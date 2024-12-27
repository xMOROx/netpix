package labeler

import (
	"fmt"
	"regexp"
	"strings"
)

type PRContext struct {
	Title        string   `json:"title"`
	Body         string   `json:"body"`
	Branch       string   `json:"branch"`
	ChangedFiles []string `json:"changed_files"`
	Files        []any    `json:"files"`
}

type MatchResult struct {
	Matched      bool     `json:"matched"`
	MatchedFiles []string `json:"matched_files,omitempty"`
	Debug        any      `json:"debug,omitempty"`
}

type Rule interface {
	Evaluate(ctx *PRContext) MatchResult
}

type RuleBuilder interface {
	Build(config map[string]any) (Rule, error)
}

type RuleRegistry struct {
	builders map[string]RuleBuilder
}

func NewRuleRegistry() *RuleRegistry {
	return &RuleRegistry{
		builders: make(map[string]RuleBuilder),
	}
}

func (r *RuleRegistry) Register(name string, builder RuleBuilder) {
	r.builders[name] = builder
}

func (r *RuleRegistry) Build(ruleType string, config map[string]any) (Rule, error) {
	if builder, ok := r.builders[ruleType]; ok {
		return builder.Build(config)
	}
	return nil, fmt.Errorf("unknown rule type: %s", ruleType)
}

func compilePatterns(patterns []string) ([]*regexp.Regexp, error) {
	compiled := make([]*regexp.Regexp, 0, len(patterns))
	for _, p := range patterns {
		p = strings.ReplaceAll(p, "(?i)", "")
		rx, err := regexp.Compile("(?i)" + p)
		if err != nil {
			return nil, err
		}
		compiled = append(compiled, rx)
	}
	return compiled, nil
}