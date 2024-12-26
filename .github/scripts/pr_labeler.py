#!/usr/bin/env python3
import sys
import json
import yaml
import argparse
from pathlib import Path
from rule_registry import RuleRegistry, RuleContext

def load_rules(rules_path):
    with open(rules_path) as f:
        return yaml.safe_load(f)

def debug_print(msg, debug=False):
    if debug:
        print(f"DEBUG: {msg}", file=sys.stderr)

def analyze_file_changes(files_data, debug=False):
    changes = {
        'total_files': len(files_data),
        'files_by_status': {},
        'files_by_extension': {},
        'changes_by_path': {}
    }
    
    for file in files_data:
        status = file.get('status', 'unknown')
        changes['files_by_status'][status] = changes['files_by_status'].get(status, 0) + 1
        
        path = Path(file['filename'])
        ext = path.suffix
        changes['files_by_extension'][ext] = changes['files_by_extension'].get(ext, 0) + 1
        
        parent = str(path.parent)
        changes['changes_by_path'][parent] = changes['changes_by_path'].get(parent, 0) + 1
        
        if debug:
            debug_print(f"File: {file['filename']}")
            debug_print(f"  Status: {status}")
            debug_print(f"  Changes: +{file.get('additions', 0)} -{file.get('deletions', 0)}")
    
    return changes

def parse_rule_config(config: list) -> list:
    rules = []
    for rule_set in config:
        if not isinstance(rule_set, dict):
            continue
            
        for rule_type, rule_config in rule_set.items():
            rule = RuleRegistry.create_rule(rule_type, {rule_type: rule_config})
            if rule:
                rules.append(rule)
    return rules

def evaluate_rules(rules_data, context, debug=False):
    matches = {}
    
    for label, config in rules_data.items():
        debug_print(f"\nEvaluating label: {label}", debug)
        if not isinstance(config, list):
            continue
            
        rules = parse_rule_config(config)
        for rule in rules:
            debug_print(f"  Checking rule: {rule.__class__.__name__}", debug)
            if rule.evaluate(context):
                debug_print("    ✅ Rule matched!", debug)
                matches[label] = {
                    'matched': True,
                    'debug_info': rule.get_debug_info()
                }
                if hasattr(rule, 'get_matched_files'):
                    matched_files = rule.get_matched_files(context)
                    matches[label]['matched_files'] = matched_files
                    debug_print(f"    Matched files: {matched_files}", debug)
                break
            else:
                debug_print("    ❌ Rule did not match", debug)
    
    return matches

def main():
    parser = argparse.ArgumentParser(description='PR Labeler')
    parser.add_argument('pr_data', help='PR data JSON file')
    parser.add_argument('--debug', action='store_true', help='Enable debug output')
    parser.add_argument('--rules', default='.github/labeler.yml', help='Path to rules file')
    args = parser.parse_args()

    with open(args.pr_data) as f:
        pr_data = json.load(f)
    
    debug_print("Loaded PR data:", args.debug)
    debug_print(json.dumps(pr_data, indent=2), args.debug)
    
    rules_data = load_rules(args.rules)
    debug_print("\nLoaded rules:", args.debug)
    debug_print(json.dumps(rules_data, indent=2), args.debug)
    
    file_analysis = analyze_file_changes(pr_data['files'], args.debug)
    debug_print("\nFile changes analysis:", args.debug)
    debug_print(json.dumps(file_analysis, indent=2), args.debug)
    
    context = RuleContext(
        pr_title=pr_data['title'],
        pr_body=pr_data['body'],
        pr_branch=pr_data['branch'],
        changed_files=pr_data['changed_files']
    )
    
    matches = evaluate_rules(rules_data, context, args.debug)
    
    result = {
        'labels': list(matches.keys()),
        'debug_info': matches,
        'file_analysis': file_analysis
    }
    
    if args.debug:
        debug_print("\nFinal result:", True)
        debug_print(json.dumps(result, indent=2), True)
    else:
        print(json.dumps(result))

if __name__ == '__main__':
    main()
