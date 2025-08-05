#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import re

# All possible English tags we want to check
english_tags = [
    'Download Security',
    'Community', 
    'Feed',
    'Resource Records',
    'User Actions', 
    'Forbidden Words',
    'Subscriptions',
    'Cache',
    'Security Management',
    'Announcements',
    'Public',
    'Schemas',
    'Auth',
    'Users',
    'Packages',
    'Comments',
    'Posts', 
    'Tags',
    'Categories',
    'Admin',
    'Health'
]

def find_english_tags_in_file(filename):
    try:
        with open(filename, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        found_tags = []
        
        for i, line in enumerate(lines, 1):
            # Look for tags: followed by - Tag patterns
            if 'tags:' in line:
                # Check the next few lines for English tags
                for j in range(i, min(i+5, len(lines))):
                    next_line = lines[j]
                    for tag in english_tags:
                        if f'- {tag}' in next_line:
                            found_tags.append({
                                'tag': tag,
                                'line_num': j+1,
                                'line_content': next_line.strip(),
                                'context_line': i
                            })
        
        return found_tags
        
    except Exception as e:
        print(f"Error processing file {filename}: {e}")
        return []

def main():
    filename = "openapi.yaml"
    found_tags = find_english_tags_in_file(filename)
    
    if found_tags:
        print(f"Found {len(found_tags)} English tags still remaining:")
        print("=" * 60)
        for item in found_tags:
            print(f"Line {item['line_num']}: {item['line_content']}")
            print(f"  Tag: {item['tag']}")
            print(f"  Context starts at line: {item['context_line']}")
            print("-" * 40)
    else:
        print("✅ No English tags found! All tags appear to be in Chinese.")
        
    # Also check the tags definition section
    print("\n" + "=" * 60)
    print("Checking tags definition section...")
    
    try:
        with open(filename, 'r', encoding='utf-8') as f:
            content = f.read()
            
        # Find the tags section
        tags_section_match = re.search(r'^tags:\s*$.*?^(?=\S|\Z)', content, re.MULTILINE | re.DOTALL)
        if tags_section_match:
            tags_section = tags_section_match.group(0)
            print("Tags definition section:")
            print(tags_section)
        else:
            print("Could not find tags definition section")
            
    except Exception as e:
        print(f"Error reading tags section: {e}")

if __name__ == "__main__":
    main() 