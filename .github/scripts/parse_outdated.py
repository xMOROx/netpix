#!/usr/bin/env python3

import sys
from typing import List, Dict

def parse_outdated(content: str) -> List[Dict[str, str]]:
    lines = content.strip().split('\n')
    if len(lines) < 3:  # Header + separator + at least one entry
        return []
    
    headers = lines[0].split()
    positions = [lines[0].index(header) for header in headers]
    positions.append(len(lines[0]) + 1)
    
    results = []
    # Skip header and separator lines
    for line in lines[2:]:
        if not line.strip():
            continue
        
        entry = {}
        for i, header in enumerate(headers):
            value = line[positions[i]:positions[i+1]].strip()
            entry[header.lower()] = value
        results.append(entry)
    
    return results

def create_markdown_report(deps: List[Dict[str, str]]) -> str:
    if not deps:
        return "No outdated dependencies found! 🎉"
    
    normal_deps = [d for d in deps if d['kind'].lower() == 'normal']
    dev_deps = [d for d in deps if d['kind'].lower() != 'normal']
    
    lines = ["# 📦 Outdated Dependencies Report\n"]
    
    if normal_deps:
        lines.append("## Production Dependencies")
        lines.append("| Package | Current | Latest | Update Size |")
        lines.append("|---------|----------|---------|-------------|")
        for dep in normal_deps:
            diff = "Major" if dep['latest'].split('.')[0] != dep['project'].split('.')[0] else \
                   "Minor" if dep['latest'].split('.')[1] != dep['project'].split('.')[1] else "Patch"
            emoji = "🚨" if diff == "Major" else "⚠️" if diff == "Minor" else "💡"
            lines.append(f"| {dep['name']} | {dep['project']} | {dep['latest']} | {emoji} {diff} |")
        lines.append("")
    
    if dev_deps:
        lines.append("## Development Dependencies")
        lines.append("| Package | Current | Latest | Update Size |")
        lines.append("|---------|----------|---------|-------------|")
        for dep in dev_deps:
            diff = "Major" if dep['latest'].split('.')[0] != dep['project'].split('.')[0] else \
                   "Minor" if dep['latest'].split('.')[1] != dep['project'].split('.')[1] else "Patch"
            emoji = "🚨" if diff == "Major" else "⚠️" if diff == "Minor" else "💡"
            lines.append(f"| {dep['name']} | {dep['project']} | {dep['latest']} | {emoji} {diff} |")
    
    return "\n".join(lines)

if __name__ == "__main__":
    content = sys.stdin.read()
    deps = parse_outdated(content)
    print(create_markdown_report(deps))
