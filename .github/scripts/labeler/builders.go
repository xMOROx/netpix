package labeler

import "fmt"

func toStringSlice(value any) ([]string, error) {
	switch v := value.(type) {
	case []string:
		return v, nil
	case []interface{}:
		result := make([]string, len(v))
		for i, item := range v {
			if str, ok := item.(string); ok {
				result[i] = str
			} else {
				return nil, fmt.Errorf("invalid pattern type at index %d: %T", i, item)
			}
		}
		return result, nil
	default:
		return nil, fmt.Errorf("invalid patterns type: %T", value)
	}
}

type BranchRuleBuilder struct{}

func (b *BranchRuleBuilder) Build(config map[string]any) (Rule, error) {
	patterns, err := toStringSlice(config["head-branch"])
	if err != nil {
		return nil, fmt.Errorf("invalid head-branch patterns: %v", err)
	}

	compiled, err := compilePatterns(patterns)
	if err != nil {
		return nil, err
	}

	return &PatternRule{
		patterns: compiled,
		field:    func(ctx *PRContext) string { return ctx.Branch },
	}, nil
}

type TitleRuleBuilder struct{}

func (b *TitleRuleBuilder) Build(config map[string]any) (Rule, error) {
	patterns, err := toStringSlice(config["title"])
	if err != nil {
		return nil, fmt.Errorf("invalid title patterns: %v", err)
	}

	compiled, err := compilePatterns(patterns)
	if err != nil {
		return nil, err
	}

	return &PatternRule{
		patterns: compiled,
		field:    func(ctx *PRContext) string { return ctx.Title },
	}, nil
}

type BodyRuleBuilder struct{}

func (b *BodyRuleBuilder) Build(config map[string]any) (Rule, error) {
	patterns, err := toStringSlice(config["body"])
	if err != nil {
		return nil, fmt.Errorf("invalid body patterns: %v", err)
	}

	compiled, err := compilePatterns(patterns)
	if err != nil {
		return nil, err
	}

	return &PatternRule{
		patterns: compiled,
		field:    func(ctx *PRContext) string { return ctx.Body },
	}, nil
}

type FilesRuleBuilder struct{}

func (b *FilesRuleBuilder) Build(config map[string]any) (Rule, error) {
	patterns, err := toStringSlice(config["changed-files"])
	if err != nil {
		return nil, fmt.Errorf("invalid file patterns: %v", err)
	}

	return &FilePatternRule{patterns: patterns}, nil
}

type CompositeRuleBuilder struct {
	Registry   *RuleRegistry
	RequireAll bool
}

func NewCompositeRuleBuilder(registry *RuleRegistry, requireAll bool) *CompositeRuleBuilder {
	return &CompositeRuleBuilder{
		Registry:   registry,
		RequireAll: requireAll,
	}
}

func (b *CompositeRuleBuilder) Build(config map[string]any) (Rule, error) {
	ruleType := "any"
	if b.RequireAll {
		ruleType = "all"
	}

	conditions, ok := config[ruleType].([]any)
	if !ok {
		return nil, fmt.Errorf("invalid %s conditions: expected array", ruleType)
	}

	rules := make([]Rule, 0)
	for _, cond := range conditions {
		condMap, ok := cond.(map[string]any)
		if !ok {
			return nil, fmt.Errorf("invalid condition: expected map")
		}

		for subRuleType, subRuleConfig := range condMap {
			var rule Rule
			var err error

			if subRuleType == "or" {
				orConfig := map[string]any{"or": subRuleConfig}
				rule, err = b.Registry.Build("or", orConfig)
			} else {
				configMap := map[string]any{subRuleType: subRuleConfig}
				rule, err = b.Registry.Build(subRuleType, configMap)
			}

			if err != nil {
				return nil, fmt.Errorf("failed to build rule %s: %v", subRuleType, err)
			}
			if rule != nil {
				rules = append(rules, rule)
			}
		}
	}

	if len(rules) == 0 {
		return nil, fmt.Errorf("no valid rules found in composite configuration")
	}

	return &CompositeRule{
		rules:      rules,
		requireAll: b.RequireAll,
	}, nil
}

type OrRuleBuilder struct {
	Registry *RuleRegistry
}

func NewOrRuleBuilder(registry *RuleRegistry) *OrRuleBuilder {
	return &OrRuleBuilder{Registry: registry}
}

func (b *OrRuleBuilder) Build(config map[string]any) (Rule, error) {
	orConfig, ok := config["or"].(map[string]any)
	if !ok {
		return nil, fmt.Errorf("invalid or configuration")
	}

	rules := make([]Rule, 0)
	for ruleType, patterns := range orConfig {
		ruleConfig := map[string]any{ruleType: patterns}
		rule, err := b.Registry.Build(ruleType, ruleConfig)
		if err != nil {
			return nil, fmt.Errorf("failed to build or sub-rule %s: %v", ruleType, err)
		}
		rules = append(rules, rule)
	}

	if len(rules) == 0 {
		return nil, fmt.Errorf("no valid rules found in or configuration")
	}

	return &CompositeRule{
		rules:      rules,
		requireAll: false,
	}, nil
}
