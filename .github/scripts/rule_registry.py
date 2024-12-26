from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import List, Dict, Any, Optional, Type, Callable
import re

@dataclass
class RuleContext:
    pr_title: str
    pr_body: str
    pr_branch: str
    changed_files: List[str]

class Rule(ABC):
    @abstractmethod
    def evaluate(self, context: RuleContext) -> bool:
        pass

    @abstractmethod
    def get_debug_info(self) -> dict:
        pass

class RuleBuilder(ABC):
    @abstractmethod
    def build(self, config: Any) -> Rule:
        pass

    @abstractmethod
    def can_build(self, config: Any) -> bool:
        pass

class RuleRegistry:
    _builders: Dict[str, RuleBuilder] = {}

    @classmethod
    def register(cls, name: str) -> Callable[[Type[RuleBuilder]], Type[RuleBuilder]]:
        def decorator(builder_cls: Type[RuleBuilder]) -> Type[RuleBuilder]:
            cls._builders[name] = builder_cls()
            return builder_cls
        return decorator

    @classmethod
    def create_rule(cls, rule_type: str, config: Any) -> Optional[Rule]:
        builder = cls._builders.get(rule_type)
        if builder and builder.can_build(config):
            return builder.build(config)
        return None

class PatternRule(Rule):
    def __init__(self, patterns: List[str], field_extractor: Callable[[RuleContext], str]):
        self.patterns = [p.replace('(?i)', '') for p in patterns]
        self.field_extractor = field_extractor
    
    def evaluate(self, context: RuleContext) -> bool:
        text = self.field_extractor(context)
        return any(re.search(pattern, text, re.IGNORECASE) for pattern in self.patterns)

    def get_debug_info(self) -> dict:
        return {'patterns': self.patterns}

class FilePatternRule(Rule):
    def __init__(self, patterns: List[str]):
        self.patterns = patterns
    
    def evaluate(self, context: RuleContext) -> bool:
        return any(
            any(re.match(f"^{pattern.replace('*', '.*')}$", file) for pattern in self.patterns)
            for file in context.changed_files
        )

    def get_matched_files(self, context: RuleContext) -> List[str]:
        return [
            file for file in context.changed_files
            if any(re.match(f"^{pattern.replace('*', '.*')}$", file) for pattern in self.patterns)
        ]

    def get_debug_info(self) -> dict:
        return {'file_patterns': self.patterns}

class CompositeRule(Rule):
    def __init__(self, rules: List[Rule], require_all: bool = True):
        self.rules = rules
        self.require_all = require_all
    
    def evaluate(self, context: RuleContext) -> bool:
        if self.require_all:
            return all(rule.evaluate(context) for rule in self.rules)
        return any(rule.evaluate(context) for rule in self.rules)

    def get_debug_info(self) -> dict:
        return {
            'type': 'all' if self.require_all else 'any',
            'rules': [rule.get_debug_info() for rule in self.rules]
        }

@RuleRegistry.register('branch')
class BranchRuleBuilder(RuleBuilder):
    def can_build(self, config: Any) -> bool:
        return isinstance(config, dict) and 'head-branch' in config

    def build(self, config: Dict) -> Rule:
        return PatternRule(config['head-branch'], lambda ctx: ctx.pr_branch)

@RuleRegistry.register('title')
class TitleRuleBuilder(RuleBuilder):
    def can_build(self, config: Any) -> bool:
        return isinstance(config, dict) and 'title' in config

    def build(self, config: Dict) -> Rule:
        return PatternRule(config['title'], lambda ctx: ctx.pr_title)

@RuleRegistry.register('body')
class BodyRuleBuilder(RuleBuilder):
    def can_build(self, config: Any) -> bool:
        return isinstance(config, dict) and 'body' in config

    def build(self, config: Dict) -> Rule:
        return PatternRule(config['body'], lambda ctx: ctx.pr_body)

@RuleRegistry.register('files')
class FilesRuleBuilder(RuleBuilder):
    def can_build(self, config: Any) -> bool:
        return isinstance(config, dict) and 'changed-files' in config

    def build(self, config: Dict) -> Rule:
        return FilePatternRule(config['changed-files'])

@RuleRegistry.register('or')
class OrRuleBuilder(RuleBuilder):
    def can_build(self, config: Any) -> bool:
        return isinstance(config, dict) and 'or' in config

    def build(self, config: Dict) -> Rule:
        rules = []
        for rule_type, rule_config in config['or'].items():
            rule = RuleRegistry.create_rule(rule_type, {rule_type: rule_config})
            if rule:
                rules.append(rule)
        return CompositeRule(rules, require_all=False)

@RuleRegistry.register('all')
class AllRuleBuilder(RuleBuilder):
    def can_build(self, config: Any) -> bool:
        return isinstance(config, dict) and 'all' in config

    def build(self, config: Dict) -> Rule:
        rules = []
        for rule_config in config['all']:
            for rule_type, inner_config in rule_config.items():
                rule = RuleRegistry.create_rule(rule_type, {rule_type: inner_config})
                if rule:
                    rules.append(rule)
        return CompositeRule(rules, require_all=True)
