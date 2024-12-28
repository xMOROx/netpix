package labeler

import (
	"fmt"
	"path/filepath"
	"regexp"
	"strings"
)

type PatternRule struct {
	patterns []*regexp.Regexp
	field    func(*PRContext) string
	ruleType string
}

func (r *PatternRule) Evaluate(ctx *PRContext) MatchResult {
	texts := []string{
		r.field(ctx),
		strings.ToLower(r.field(ctx)),
	}

	for _, text := range texts {
		for _, p := range r.patterns {
			if p.MatchString(text) {
				return NewMatchResult(true, r.ruleType,
					fmt.Sprintf("Pattern '%s' matched text: %s", p.String(), text))
			}
		}
	}

	return NewMatchResult(false, r.ruleType,
		fmt.Sprintf("No patterns matched text: %s", texts[0]))
}

type FilePatternRule struct {
	patterns []string
}

func (r *FilePatternRule) Evaluate(ctx *PRContext) MatchResult {
	matchedFiles := make([]string, 0)
	result := NewMatchResult(false, "changed-files", "")

	for _, file := range ctx.ChangedFiles {
		for _, pattern := range r.patterns {
			pattern = strings.ReplaceAll(pattern, "**", "*")
			isMatched, err := filepath.Match(pattern, file)
			if err != nil {
				isMatched = pattern == file
			}
			if isMatched {
				matchedFiles = append(matchedFiles, file)
				result.Description += fmt.Sprintf("File %s matched pattern %s\n", file, pattern)
				break
			}
		}
	}

	result.Matched = len(matchedFiles) > 0
	result.MatchedFiles = matchedFiles
	if !result.Matched {
		result.Description = fmt.Sprintf("No files matched patterns: %v\nFiles checked: %v", r.patterns, ctx.ChangedFiles)
	}
	return result
}

type CompositeRule struct {
	rules      []Rule
	requireAll bool
	ruleType   string
}

func (r *CompositeRule) Evaluate(ctx *PRContext) MatchResult {
	if r.requireAll {
		return r.evaluateAll(ctx)
	}
	return r.evaluateAny(ctx)
}

func (r *CompositeRule) evaluateAll(ctx *PRContext) MatchResult {
	result := NewMatchResult(true, r.ruleType, "Evaluating ALL rules")
	result.SubResults = make([]*MatchResult, 0)

	for _, rule := range r.rules {
		subResult := rule.Evaluate(ctx)
		result.SubResults = append(result.SubResults, &subResult)
		if !subResult.Matched {
			result.Matched = false
			result.Description = "Not all rules matched"
			return result
		}
		result.MatchedFiles = append(result.MatchedFiles, subResult.MatchedFiles...)
	}

	result.Description = "All rules matched successfully"
	return result
}

func (r *CompositeRule) evaluateAny(ctx *PRContext) MatchResult {
	result := NewMatchResult(false, r.ruleType, "Evaluating ANY rules")
	result.SubResults = make([]*MatchResult, 0)

	for _, rule := range r.rules {
		subResult := rule.Evaluate(ctx)
		result.SubResults = append(result.SubResults, &subResult)
		if subResult.Matched {
			result.Matched = true
			result.Description = "At least one rule matched"
			result.MatchedFiles = subResult.MatchedFiles
			return result
		}
	}

	result.Description = "No rules matched"
	return result
}
