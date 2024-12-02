# advent-of-code
My solutions for https://adventofcode.com.

- [2024](https://github.com/nlbao/advent-of-code/tree/main/2024/src/bin/) in Rust :crab:


## Notes

To make VSCode rust-analyzer works for Rust files, you need to:
* Store Rust files in **{year}/src/bin/**.
* Add the below to VS code workspace settings:

```json
    "rust-analyzer.linkedProjects": [
        "{path-to-repo}/advent-of-code/{year}/Cargo.toml" 
    ],
```
