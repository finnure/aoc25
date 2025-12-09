# AOC 2025

[Advent of Code](https://adventofcode.com/) is an annual online event featuring daily programming puzzles throughout December. Participants solve challenges to practice coding skills, compete with others, and enjoy festive problem-solving.

This year, I'm learning [Rust](https://www.rust-lang.org/) and using the Advent of Code puzzles to improve my skills.

## How to run

To run the project for a specific day, use the following command in your terminal:

```sh
cargo run -p runner <day>
```

Replace `<day>` with the desired day number to run the corresponding solution. This command builds and runs the project using Cargo, targeting the `runner` package.

## Puzzle summaries

### Day 1

The input consists of instructions to rotate a dial numbered from 0 to 99. Each instruction begins with either 'L' (rotate left) or 'R' (rotate right), followed by a number indicating how many positions to rotate. The dial starts at position 50.

#### Part 1

Determine how many instructions cause the dial to land exactly on position 0.

#### Part 2

Calculate how many times the dial passes over position 0 while following all the instructions.

### Day 2

The input consists of ranges of IDs separated by commas.

#### Part 1

Identify invalid IDs where an invalid ID consists of two numbers repeated consecutively (e.g., `6464` is invalid because `64` appears twice).

#### Part 2

An ID is invalid if it is made up entirely of a sequence of digits repeated at least twice. Examples include `12341234` (`1234` repeated twice), `123123123` (`123` repeated three times), `1212121212` (`12` repeated five times), and `1111111` (`1` repeated seven times).

### Day 3

The input lists battery banks, one per line. Each number represents a battery's power level.

#### Part 1

For each bank, select the two highest power cells and sum their values. The total for each bank is the sum of these two cells, listed in input order.

#### Part 2

For each bank, select the twelve highest power cells and sum their values. The total for each bank is the sum of these twelve cells, listed in input order.

### Day 4

The input displays the arrangement of paper rolls, represented by `@` symbols.

#### Part 1

Count how many rolls are accessible with a forklift. A roll is accessible if fewer than four rolls occupy the eight adjacent positions (up, down, left, right, and the four diagonals).

#### Part 2

Repeatedly remove accessible rolls using the same rule as in Part 1, and count the total number of rolls that can be removed.

### Day 5

The input consists of two sections separated by a blank line. The first section lists ranges of numbers (e.g., `3-5`), and the second section lists individual numbers.

#### Part 1

Count how many numbers from the second section fall within any of the ranges specified in the first section.

#### Part 2

Merge overlapping or adjacent ranges from the first section, then sum the total number of distinct values covered by these merged ranges.

### Day 6

The input consists of several rows of numbers, followed by a row of operators (`+` or `*`). Each column is processed according to its operator.

#### Part 1

For each column, sum or multiply the numbers above it depending on whether the operator is `+` (sum) or `*` (product). The final answer is the sum of all these results.

#### Part 2

Process the columns from right to left. For each column, concatenate the digits from each row to form a number, then sum or multiply as in Part 1, but reset the sum and product when a column cannot be parsed as a number. The final answer is the sum of all these results.

### Day 7

The input is a grid of characters, where `S` marks a starting position of a particle and `^` marks switches where the particle splits in two.

#### Part 1

Count how many switches the particle can hit.

#### Part 2

Count how many different tracks the particle can travel down the grid.

### Day 8

The input is a list of 3D points, each with `x`, `y`, and `z` coordinates.

#### Part 1

Calculate the Euclidean distance between all pairs of points. Connect points starting with the pair with the shortest distance, building circuits as you go. After processing the first 1000 pairs, identify the three largest circuits and multiply the number of points in each.

#### Part 2

Continue connecting points by shortest distance. Find the pair that results in all points being connected into a single circuit. The answer is the product of the `x` positions of the two points that completed the circuit.

### Day 9

The input consists of a list of 2D coordinates representing the positions of red tiles arranged in a pattern.

#### Part 1

Identify the pair of red tiles that serve as opposite corners of the largest possible square that can be formed using any two red tiles as opposite corners, regardless of whether the square lies entirely within the pattern.

#### Part 2

The input describes a closed shape formed by straight lines connecting a sequence of points, with the last point connected back to the first. Find the largest square that can be placed inside the shape such that two of its opposite corners coincide with red tiles.
