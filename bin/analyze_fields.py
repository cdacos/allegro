#!/usr/bin/env python3

import os
import re
import csv
import sys

def extract_field_info(file_path):
    """Extract field information from a Rust record file."""
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Extract record type from struct name
    struct_match = re.search(r'pub struct (\w+)Record', content)
    if not struct_match:
        return []
    
    record_type = struct_match.group(1).upper()
    
    fields = []
    processed_fields = set()  # Track processed fields to avoid duplicates
    lines = content.split('\n')
    
    i = 0
    while i < len(lines):
        line = lines[i].strip()
        
        # Look for cwr attribute
        if line.startswith('#[cwr('):
            attr_line = line
            
            # Continue reading if the attribute spans multiple lines
            while not attr_line.endswith(')]') and i < len(lines) - 1:
                i += 1
                attr_line += ' ' + lines[i].strip()
            
            # Parse the attribute
            start_match = re.search(r'start\s*=\s*(\d+)', attr_line)
            min_version_match = re.search(r'min_version\s*=\s*([0-9.]+)', attr_line)
            title_match = re.search(r'title\s*=\s*"([^"]*)"', attr_line)
            
            # Look for the field declaration in the next few lines
            j = i + 1
            while j < len(lines) and j < i + 5:  # Look ahead max 5 lines
                field_line = lines[j].strip()
                if field_line.startswith('pub ') and ':' in field_line:
                    # Parse field declaration
                    field_match = re.search(r'pub\s+(\w+):\s*([^,\n}]+)', field_line)
                    if field_match:
                        field_name = field_match.group(1)
                        data_type = field_match.group(2).strip().rstrip(',')
                        
                        # Create unique key to avoid duplicates
                        start_index = start_match.group(1) if start_match else '0'
                        field_key = f"{record_type}:{field_name}:{start_index}"
                        
                        if field_key not in processed_fields:
                            processed_fields.add(field_key)
                            
                            # Clean up data type
                            data_type = clean_data_type(data_type)
                            version = min_version_match.group(1) if min_version_match else '2.0'
                            
                            fields.append({
                                'RecordType': record_type,
                                'Field': field_name,
                                'StartIndex': start_index,
                                'DataType': data_type,
                                'CWR_Version': version
                            })
                    break
                j += 1
        
        i += 1
    
    return fields

def clean_data_type(data_type):
    """Clean up Rust data types for CSV output."""
    # Remove Option wrapper
    data_type = re.sub(r'Option<(.+)>', r'\1', data_type)
    # Remove Vec wrapper  
    data_type = re.sub(r'Vec<(.+)>', r'\1', data_type)
    # Clean up whitespace
    data_type = data_type.strip()
    return data_type

def main():
    records_dir = 'crates/allegro_cwr/src/records'
    
    if not os.path.exists(records_dir):
        print(f"Error: Directory {records_dir} not found", file=sys.stderr)
        return 1
    
    all_fields = []
    
    # Process all .rs files in the records directory
    for filename in os.listdir(records_dir):
        if filename.endswith('.rs') and filename != 'mod.rs':
            file_path = os.path.join(records_dir, filename)
            fields = extract_field_info(file_path)
            all_fields.extend(fields)
    
    # Sort by record type and start index
    all_fields.sort(key=lambda x: (x['RecordType'], int(x['StartIndex'])))
    
    # Output CSV
    if all_fields:
        fieldnames = ['RecordType', 'Field', 'StartIndex', 'DataType', 'CWR_Version']
        writer = csv.DictWriter(sys.stdout, fieldnames=fieldnames)
        writer.writeheader()
        for field in all_fields:
            writer.writerow(field)
    
    return 0

if __name__ == '__main__':
    sys.exit(main())