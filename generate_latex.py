import re
import os

MAPPING_PAPER = {
    'C1': '(C1) Not applicable: C++ Standard Library Usage Restriction',
    'C2': '(C2) Not applicable: C++ Feature does not exist in Rust',
    'C5': '(C5) Rust-specific adaptation potentially required',
    'C6': '(C6) Rule still required (safe Rust)',
    'C4': '(C4) Rule still required (unsafe Rust)',
    'C3': '(C3) Rule can be dropped completely'
}

def parse_rules(file_path):
    rules = []
    skipped_counter = 0
    with open(file_path, 'r') as file:
        for line in file:
            match = re.match(r'(\d+\.\d+\.\d+);([a-zA-Z0-9,]+);(.+)', line)
            if match:
                identifier = match.group(1)
                analysis_result = match.group(2)
                comment = match.group(3)
                rules.append({
                    'identifier': identifier,
                    'analysis_result': analysis_result,
                    'comment': comment
                })
            else:
                skipped_counter += 1
                print(f"Skipped line: {line.strip()}")
    print(f"Skipped {skipped_counter} lines that did not match the expected format.")
    return rules

def generate_latex(guidelines, output_file, latex_section_type='subsection*'):

    with open(output_file, 'w') as file:
        for gl in guidelines:
            
            # check if guideline is a dir instead of rule
            gl_type = "Rule"
            if gl['identifier'] in ['0.3.1', '0.3.2', '5.7.2', '15.8.1']:
                gl_type = "Dir"
                
            file.write('\\' + f"{latex_section_type}"+ "{" + f"{gl_type} " + gl['identifier'] + '}' + '\n')

            # Replace analysis result encoding with full text
            analysis_result_full = MAPPING_PAPER.get(gl['analysis_result'], f'Unknown Analysis Result for shorthand {gl["analysis_result"]}')
            
            # Create table
            file.write(r'\begin{longtable}{p{13cm}}' + '\n')
            file.write(r'\textbf{Analysis Result} \\' + '\n')
            file.write(analysis_result_full + r' \\' + '\n')
            file.write(r'\end{longtable}' + '\n')
            file.write(r'\textbf{Comment:} ' + gl['comment'] + r' \\' + '\n')


            rust_file = os.path.join('misra_cpp_examples', gl['identifier'].replace('.', '_') + '.rs')
            if os.path.isfile(rust_file):
                rust_file = rust_file.replace('\\', '/')
                file.write(r'\lstinputlisting[style=ruststyle, caption=Rust Code Example for guideline ' + gl['identifier'] + ', label=lst:' + gl['identifier'].replace('.', '_') + ']{' + rust_file + '}'+  '\n')
            else:
                file.write("%no example exists")

            file.write('\n')

def filter_for_identifier(identifiers, rules):
    remaining_rules = []
    for rule in rules:
        if rule['identifier'] in identifiers:
            remaining_rules.append(rule)
    return remaining_rules




if __name__ == "__main__":
    input_file = 'misra_cpp_rust_comparison_rules.csv'
    output_file = 'all_guidelines_blob.tex'
    rules = parse_rules(input_file)

    generate_latex(rules, output_file)

    # generate one latex entry for each rule
    # create directory
    os.makedirs('generated_all_guidelines', exist_ok=True)
    for r in rules:
        generate_latex([r], os.path.join('generated_all_guidelines', r['identifier'].replace('.', '_') + '.tex'), latex_section_type='paragraph')