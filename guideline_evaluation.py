# Category mapping based on the provided shorthand
MAPPING_PAPER = {
    'C1': '(C1) Not applicable: C++ Standard Library Usage Restriction',
    'C2': '(C2) Not applicable: C++ Feature does not exist in Rust',
    'C5': '(C5) Rust-specific adaptation potentially required',
    'C6': '(C6) Rule still required (safe Rust)',
    'C4': '(C4) Rule still required (unsafe Rust)',
    'C3': '(C3) Rule can be dropped completely'
}

# Topic mapping based on the provided identifier
TOPIC_MAPPING = {
    '0': 'Language independent issues',
    '4': 'General principles',
    '5': 'Lexical conventions',
    '6': 'Basic concepts',
    '7': 'Standard conversions',
    '8': 'Expressions',
    '9': 'Statements',
    '10': 'Declarations',
    '11': 'Declarators',
    '12': 'Classes',
    '13': 'Derived classes',
    '14': 'Member access control',
    '15': 'Special member functions',
    '16': 'Overloading',
    '17': 'Templates',
    '18': 'Exception handling',
    '19': 'Preprocessing directives',
    '21': 'Language support library',
    '22': 'Diagnostics library',
    '23': 'General utilities library',
    '24': 'Strings library',
    '25': 'Localization library',
    '26': 'Containers library',
    '28': 'Algorithms library',
    '30': 'Input/output library'
}

# Parse the identifier to extract the topic
def get_topic_from_identifier(identifier):
    topic_key = identifier.split('.')[0]
    return TOPIC_MAPPING.get(topic_key, 'Unknown Topic')

# parse line wise and identify correct syntax
def parse_line(line):
    parts = line.strip().split(';')
    if len(parts) >= 1:
        if parts[0].startswith("//"):
            return None
     
    if len(parts) < 3:
        if parts[0] == "":
            print("Error: Empty line encountered")
            return None
        print(f"Error: Invalid line format in line {line}")
        print(f"Expected at least 3 parts, got {len(parts)}")
        return None
    
    identifier = parts[0]
    categories = parts[1].split(',')
    comment = parts[2]
    
    topic = get_topic_from_identifier(identifier)
    
    # Map category shorthand to full names
    mapped_categories = [MAPPING_PAPER.get(cat, 'Unknown Category') for cat in categories]
    assert mapped_categories[0] != 'Unknown Category', f"Unknown category: {categories[0]} in line {line}"
    
    return {
        'identifier': identifier,
        'topic': topic,
        'categories': mapped_categories,
        'raw_category' : categories,
        'comment': comment
    }

def process_file(file_path):
    statistics = {
        'topics': {},
        'categories': {}
    }

    satisfied_rules_count = {topic: 0 for topic in TOPIC_MAPPING.values()}
    still_relevant_rules_count = {topic: 0 for topic in TOPIC_MAPPING.values()}
    total_rules_count = {topic: 0 for topic in TOPIC_MAPPING.values()}
    total_lines = 0
    all_identifier = []
    try:
        with open(file_path, 'r') as file:
            for line_number, line in enumerate(file, start=1):
                try:
                    parsed_data = parse_line(line)
                except Exception as e:
                    print(f"Error: Failed to parse line {line_number}")
                    continue
                
                
                if parsed_data:
                    all_identifier.append(parsed_data['identifier'])

                    total_lines += 1
                    # Update topic statistics
                    topic = parsed_data['topic']
                    if topic in statistics['topics']:
                        statistics['topics'][topic] += 1
                    else:
                        statistics['topics'][topic] = 1
                         

                    for rc in parsed_data['raw_category']:
                        if rc in ['C5']:
                            still_relevant_rules_count[topic] += 1
                        if rc in ['C3','C4']:
                            satisfied_rules_count[topic] += 1
                        if rc in ['C4','C6']:
                            still_relevant_rules_count[topic] += 1
                        total_rules_count[topic] += 1

                        pass

                    # Update category statistics
                    for category in parsed_data['categories']:
                        #print(f"Category is f: {category}")
                        if category in statistics['categories']:
                            statistics['categories'][category] += 1
                        else:
                            statistics['categories'][category] = 1


        # C4 := Sum of C4, C6 :: C6 is a subset of C4, so we add the count of C6 to C4 to get the total count of rules that are still relevant in unsafe Rust.
        # This is because all rules that are still relevant in safe Rust (C6) are also relevant in unsafe Rust (C4), 
        # but there may be additional rules that are relevant in unsafe Rust that are not relevant in safe Rust. 
        # By adding the counts together, we ensure that we account for all rules that are still relevant in unsafe Rust, including those that are also relevant in safe Rust.
        statistics['categories'][MAPPING_PAPER['C4']] += statistics['categories'][MAPPING_PAPER['C6']]
        print(f"Found a total of {total_lines} guidelines")
                
        print(all_identifier)
        print(len(all_identifier))
        # Print out statistics
        print("Topic Distribution:")
        for topic, count in statistics['topics'].items():
            print(f"{topic}: {count} guidelines")

        print("\nCategory Distribution (with percentages):")
        for category, count in statistics['categories'].items():
            percentage = (count / total_lines) * 100
            print(f"{category}: {count} guidelines ({percentage:.2f}%)")
    
        print("\nStill relevant guidelines Count:")
        for topic, count in still_relevant_rules_count.items():
            if count == 0:
                continue
            total = total_rules_count[topic]
            if int(total) > 0:
                percent = round(int(count)/int(total),3)
            print(f"{topic}: {count}/{total} equals {percent*100}% rules for this topic")
    except Exception as e:
        print(f"Error during parsing '{e}'.")

# Example usage
if __name__ == "__main__":
    file_path = 'misra_cpp_rust_comparison_rules.csv'
    process_file(file_path)