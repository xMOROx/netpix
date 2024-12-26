from dataclasses import dataclass
from typing import List, Dict, Any, Optional
import re
from fnmatch import fnmatch
from pathlib import Path

@dataclass
class RuleContext:
    pr_title: str
    pr_body: str
    pr_branch: str
    changed_files: List[str]

class RuleEvaluator:
    def __init__(self, context: RuleContext):
        self.context = context

    def matches_pattern(self, text: str, patterns: List[str]) -> bool:
        if not patterns:
            return False
        return any(re.search(pattern.replace('(?i)', ''), text, re.IGNORECASE) 
                  for pattern in patterns)

    def matches_file_pattern(self, patterns: List[str]) -> bool:
        if not patterns or not self.context.changed_files:
            return False
        return any(
            any(fnmatch(file, pattern) for pattern in patterns)
            for file in self.context.changed_files
        )

    def get_matched_files(self, patterns: List[str]) -> List[str]:
        return [
            file for file in self.context.changed_files
            if any(fnmatch(file, pattern) for pattern in patterns)
        ]

    def evaluate_or_condition(self, condition: Dict[str, List[str]]) -> bool:
        if not condition:
            return False

        return (
            self.matches_pattern(self.context.pr_branch, condition.get('head-branch', [])) or
            self.matches_pattern(self.context.pr_title, condition.get('title', [])) or
            self.matches_pattern(self.context.pr_body, condition.get('body', []))
        )

    def evaluate_rule(self, rule_config: List[Dict]) -> Dict[str, Any]:
        for rule_set in rule_config:
            if 'any' in rule_set:
                conditions = rule_set['any']
                or_conditions = next((cond['or'] for cond in conditions if 'or' in cond), {})
                file_patterns = next((cond['changed-files'] for cond in conditions if 'changed-files' in cond), [])
                
                matches_metadata = self.evaluate_or_condition(or_conditions)
                matches_files = self.matches_file_pattern(file_patterns)
                
                if matches_metadata and matches_files:
                    return {
                        'matched': True,
                        'matched_files': self.get_matched_files(file_patterns),
                        'matched_metadata': {
                            'branch': self.matches_pattern(self.context.pr_branch, or_conditions.get('head-branch', [])),
                            'title': self.matches_pattern(self.context.pr_title, or_conditions.get('title', [])),
                            'body': self.matches_pattern(self.context.pr_body, or_conditions.get('body', []))
                        }
                    }
                
        return {'matched': False}

class RuleParser:
    @staticmethod
    def parse_rules(rules_data: Dict[str, Any], context: RuleContext) -> Dict[str, Any]:
        evaluator = RuleEvaluator(context)
        matches = {}
        
        for label, rule_config in rules_data.items():
            if not isinstance(rule_config, list):
                continue

            result = evaluator.evaluate_rule(rule_config)
            if result['matched']:
                matches[label] = result

        return matches
