# usask_cba_calc

A command-line tool designed to calculate the first-year engineering grades for students at the University of Saskatchewan's College of Engineering. 
This tool takes an input file in JSON format and provides the calculated GPA and other relevant information. The tool is written in Rust to take advantage
of Rust's excellent data modelling capabilities.

## Installation

1. Install Rust: If you don't already have Rust installed on your system, you can do so by visiting the [official Rust website](https://www.rust-lang.org/tools/install) and following their installation instructions.

2. Install the Binary: Use cargo to install the binary from crates.io
`cargo install usask_cba_calculator`

3. Verify Installation: Run the following command in your terminal
`usask_cba_calc --help`

The output should look something like this,
```
Usage: usask-cba-calc [file_path]

Arguments:
    [file_path]  Path to file containing JSON data.

If no file path is provided, the program will read from stdin.


cli tool to calculate usask's first year engineering cba grades

usask-cba-calc v0.2.3
```

## Usage

1. Open up your terminal or command prompt

2. Type `usask_cba_calc -s`  

    This will ask you a few questions to generate a boiler plate JSON schema, later open the grades.json file to fill in your grade information

3. Type `usask_cba_calc /path/to/file.json` 

NOTE: You can also pipe in data to the tool.

## Input file schema

The way the input file needs to be structured is very specific, follow the general layout shown below

```
{
    "Subject1": {
        "CLOs": {
            "1.1": {
                "difficulty_type": "B",
                "weightage": 20.0,
                "RLOs": {
                    "1.3": {
                        "weightage": 50.0,
                        "assignments": [95, 90, 85]
                    },
                    "3.3": {
                        "weightage": 50.0,
                        "assignments": [80, 75]
                    }
                }
            },
            "1.2": {
                "difficulty_type": "B+",
                "weightage": 80.0,
                "RLOs": {
                    "1.1": {
                        "weightage": 50.0,
                        "assignments": [95, 90, 85]
                    }
                }
            }
        }
    },
    "Subject2": {
        "CLOs": {
            "2.1": {
                "difficulty_type": "A",
                "weightage": 0.0,
                "RLOs": {
                    "2.1": {
                        "weightage": 100.0,
                        "assignments": [12]
                    }
                }
            },
            "1.2": {
                "difficulty_type": "B+",
                "weightage": 100.0,
                "RLOs": {
                    "2.2": {
                        "weightage": 100.0,
                        "assignments": [95, 0, 0]
                    }
                }
            }
        }
    }
}
```


