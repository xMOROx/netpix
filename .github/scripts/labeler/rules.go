package labeler

import (
	"path/filepath"
	"regexp"
)

type PatternRule struct {
	patterns []*regexp.Regexp
	field    func(*PRContext) string
}

func (r *PatternRule) Evaluate(ctx *PRContext) MatchResult {
	text := r.field(ctx)
	for _, p := range r.patterns {
		if p.MatchString(text) {
			return MatchResult{Matched: true}
		}
	}
	return MatchResult{Matched: false}
}

type FilePatternRule struct {
	patterns []string
}

func (r *FilePatternRule) Evaluate(ctx *PRContext) MatchResult {
	matched := make([]string, 0)
	for _, file := range ctx.ChangedFiles {
		for _, pattern := range r.patterns {
			if ok, _ := filepath.Match(pattern, file); ok {
				matched = append(matched, file)
				break
			}
		}
	}
	return MatchResult{
		Matched:      len(matched) > 0,
		MatchedFiles: matched,
	}
}

type CompositeRule struct {
	rules       []Rule
	requireAll  bool
	matchedRule Rule
}

func (r *CompositeRule) Evaluate(ctx *PRContext) MatchResult {
	if r.requireAll {
		return r.evaluateAll(ctx)
	}
	return r.evaluateAny(ctx)
}

func (r *CompositeRule) evaluateAll(ctx *PRContext) MatchResult {
	var matchedFiles []string
	for _, rule := range r.rules {
		result := rule.Evaluate(ctx)
		if !result.Matched {
			return MatchResult{Matched: false}
		}
		matchedFiles = append(matchedFiles, result.MatchedFiles...)
	}
	return MatchResult{
		Matched:      true,
		MatchedFiles: matchedFiles,
	}
}

func (r *CompositeRule) evaluateAny(ctx *PRContext) MatchResult {
	for _, rule := range r.rules {
		result := rule.Evaluate(ctx)
		if result.Matched {
			return result
		}
	}
	return MatchResult{Matched: false}
}
