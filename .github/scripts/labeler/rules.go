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
					fmt.Sprintf("[MATCHED] Rule type: %s, Pattern '%s' matched text: %s", r.ruleType, p.String(), text))
			}
		}
	}

	return NewMatchResult(false, r.ruleType,
		fmt.Sprintf("[NOT MATCHED] Rule type: %s, No patterns matched. Text: %s, Patterns: %v",
			r.ruleType, texts[0], r.patterns))
}

type FilePatternRule struct {
	patterns []string
}

func (r *FilePatternRule) Evaluate(ctx *PRContext) MatchResult {
	matchedFiles := make([]string, 0)
	result := NewMatchResult(false, "changed-files", "")
	var matchDetails strings.Builder

	matchDetails.WriteString(fmt.Sprintf("Checking files against patterns: %v\n", r.patterns))

	for _, file := range ctx.ChangedFiles {
		for _, pattern := range r.patterns {
			pattern = strings.ReplaceAll(pattern, "**", "*")
			isMatched, err := filepath.Match(pattern, file)
			if err != nil {
				isMatched = pattern == file
			}
			if isMatched {
				matchedFiles = append(matchedFiles, file)
				matchDetails.WriteString(fmt.Sprintf("[MATCHED] File %s matched pattern %s\n", file, pattern))
				break
			} else {
				matchDetails.WriteString(fmt.Sprintf("[NOT MATCHED] File %s did not match pattern %s\n", file, pattern))
			}
		}
	}

	result.Matched = len(matchedFiles) > 0
	result.MatchedFiles = matchedFiles
	result.Description = matchDetails.String()
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
	result := NewMatchResult(true, r.ruleType, fmt.Sprintf("\n=== Evaluating ALL rules ===\n"))
	result.SubResults = make([]*MatchResult, 0)
	var details strings.Builder

	for i, rule := range r.rules {
		subResult := rule.Evaluate(ctx)
		result.SubResults = append(result.SubResults, &subResult)
		details.WriteString(fmt.Sprintf("Rule #%d (%s): %s\n", i+1, subResult.RuleType, subResult.Description))

		if !subResult.Matched {
			result.Matched = false
			details.WriteString("=== ALL rules evaluation failed ===\n")
			result.Description = details.String()
			return result
		}
		result.MatchedFiles = append(result.MatchedFiles, subResult.MatchedFiles...)
	}

	details.WriteString("=== ALL rules matched successfully ===\n")
	result.Description = details.String()
	return result
}

func (r *CompositeRule) evaluateAny(ctx *PRContext) MatchResult {
	result := NewMatchResult(false, r.ruleType, fmt.Sprintf("\n=== Evaluating ANY rules ===\n"))
	result.SubResults = make([]*MatchResult, 0)
	var details strings.Builder

	for i, rule := range r.rules {
		subResult := rule.Evaluate(ctx)
		result.SubResults = append(result.SubResults, &subResult)
		details.WriteString(fmt.Sprintf("Rule #%d (%s): %s\n", i+1, subResult.RuleType, subResult.Description))

		if subResult.Matched {
			result.Matched = true
			result.MatchedFiles = subResult.MatchedFiles
			details.WriteString("=== At least one rule matched ===\n")
			result.Description = details.String()
			return result
		}
	}

	details.WriteString("=== No rules matched ===\n")
	result.Description = details.String()
	return result
}
