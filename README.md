# MISRust: Mapping MISRA-C++ Coding Guidelines to the Rust Programming Language

This repository contains original research artifacts and metadata derived from analysis of the MISRA C++ 2023 guidelines. MISRA materials remain copyrighted by their respective owners and are not redistributed under this repository’s license.

No MISRA documents are included in this repository.

## Overview
Scripts contained in this directory create LaTeX code from the machine-readable document containing the complete guideline analysis (`misra_cpp_rust_comparison_rules.csv`).

## Generating Output
Simply call the scripts to generate output and list statistics.

To generate LaTeX output:
```
python generate_latex.py
pdflatex classified_rules.tex
```

To get statistics used in the publication:
```
python guideline_evaluation.py
```

The already generated files by the scripts are contained in this directory as well.

## License
This work is licensed under the [CC-BY-4.0](https://github.com/embedded-software-laboratory/MISRust/blob/main/LICENSE) license.

