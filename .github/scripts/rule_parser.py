from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import List, Dict, Any, Optional
import re
from fnmatch import fnmatch

@dataclass
class RuleContext:
    pr_title: str
    pr_body: str
    pr_branch: str
    changed_files: List[str]

class Condition(ABC):
    @abstractmethod
    def evaluate(self, context: RuleContext) -> bool:
        pass

class Pattern(ABC):
    def __init__(self, patterns: List[str]):
        self.patterns = [p.replace('(?i)', '') for p in patterns]
    
    def matches(self, text: str) -> bool:
        return any(re.search(pattern, text, re.IGNORECASE) for pattern in self.patterns)

class FilePattern(Pattern):
    def matches(self, filename: str) -> bool:
        return any(
            fnmatch(filename, pattern)
            for pattern in self.patterns
        )

class BranchPattern(Pattern):
    def evaluate(self, context: RuleContext) -> bool:
        return self.matches(context.pr_branch)

class TitlePattern(Pattern):
    def evaluate(self, context: RuleContext) -> bool:
        return self.matches(context.pr_title)

class BodyPattern(Pattern):
    def evaluate(self, context: RuleContext) -> bool:
        return self.matches(context.pr_body)

class FilesCondition(Condition):
    def __init__(self, patterns: List[str]):
        self.pattern = FilePattern(patterns)
    
    def evaluate(self, context: RuleContext) -> bool:
        return any(self.pattern.matches(file) for file in context.changed_files)

    def get_matched_files(self, context: RuleContext) -> List[str]:
        return [f for f in context.changed_files if self.pattern.matches(f)]

class OrCondition(Condition):
    def __init__(self, rules: Dict[str, List[str]]):
        self.conditions = []
        if 'head-branch' in rules:
            self.conditions.append(BranchPattern(rules['head-branch']))
        if 'title' in rules:
            self.conditions.append(TitlePattern(rules['title']))
        if 'body' in rules:
            self.conditions.append(BodyPattern(rules['body']))
    
    def evaluate(self, context: RuleContext) -> bool:
        return any(condition.evaluate(context) for condition in self.conditions)

class AllCondition(Condition):
    def __init__(self, conditions: List[Condition]):
        self.conditions = conditions
    
    def evaluate(self, context: RuleContext) -> bool:
        return all(condition.evaluate(context) for condition in self.conditions)

class AnyCondition(Condition):
    def __init__(self, conditions: List[Condition]):
        self.conditions = conditions
    
    def evaluate(self, context: RuleContext) -> bool:
        return any(condition.evaluate(context) for condition in self.conditions)

class RuleParser:
    @staticmethod
    def parse_condition(rule_data: Dict[str, Any]) -> Optional[Condition]:
        if 'all' in rule_data:
            conditions = []
            for item in rule_data['all']:
                if 'or' in item:
                    conditions.append(OrCondition(item['or']))
                elif 'changed-files' in item:
                    conditions.append(FilesCondition(item['changed-files']))
            return AllCondition(conditions)
        elif 'any' in rule_data:
            conditions = []
            for item in rule_data['any']:
                if 'or' in item:
                    conditions.append(OrCondition(item['or']))
                elif 'changed-files' in item:
                    conditions.append(FilesCondition(item['changed-files']))
            return AnyCondition(conditions)
        elif 'or' in rule_data:
            return OrCondition(rule_data['or'])
        return None

    @staticmethod
    def parse_rules(rules_data: Dict[str, Any]) -> Dict[str, List[Condition]]:
        parsed_rules = {}
        for label, config in rules_data.items():
            if not isinstance(config, list):
                continue
                
            conditions = []
            for rule_set in config:
                if not isinstance(rule_set, dict):
                    continue
                    
                condition = RuleParser.parse_condition(rule_set)
                if condition:
                    conditions.append(condition)
                    
            if conditions:
                parsed_rules[label] = conditions
                
        return parsed_rules
