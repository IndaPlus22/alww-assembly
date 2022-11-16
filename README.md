# alww-task-9

# Feng Shui (.fesu) Language Documentation

## Elements

There are only four elements available in this language: `火`, `水`, `地` and `木` for the gods have banished `銀` for this world, only imbalance remains.

## Instructions

| Colors | Syntax                   | Explanation                                                                    |
| ------ | ------------------------ | ------------------------------------------------------------------------------ |
| `黒`   | `黒 rs rt rd`            | Set value of `rs` to value of `rt` + value of `rd`                             |
| `緑`   | `緑 rs rt imm`           | Set value of `rs` to value of `rt` + immediate `imm`                           |
| `白`   | `白 rs imm`              | Set value of `rs` to immediate `imm`                                           |
| `紫`   | `紫 rs/imm rt/imm label` | Jump 1-bit unsigned rows to a label if `rs/imm` is equal to immediate `rt/imm` |
| `赤`   | `赤 imm`                 | Jump `imm` 5-bit signed rows                                                   |
| `読`   | `読`                     | Takes user input and puts it in element `木`                                   |
| `書`   | `書`                     | Prints value of element `火`                                                   |
| `殺`   | `殺`                     | Terminates the program                                                         |

## Running the interpreter

| Command                            | Syntax                            | Explanation                                        |
| ---------------------------------- | --------------------------------- | -------------------------------------------------- |
| cargo run                          | `cargo run filename.fesu execute` | `Execute` the `filename.fesu` program              |
| ------                             | -------------                     | -------------------------------------------------- |
| Example                            |
| `cargo run factorial.fesu execute` |
