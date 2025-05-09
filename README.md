# Maze Parse

This Rust project is a demonstration for parsing and representing ASCII mazes. It provides functionality to read, validate, and parse mazes into structured data.

## Features

- **Maze Representation**:

  - Represents mazes as a 2D grid of `Cell` objects.
  - Each `Cell` contains information about its walls, corners, and inner text.

- **Parsing ASCII Mazes**:

  - Supports parsing small (`parse_sm`) and large (`parse_lg`) mazes.
  - Handles overlapping walls between adjacent cells for accurate parsing.

## Example Usage

```rust
fn main() {
    let maze_str = "\
+---+---+
| A | B |
+---+---+
| C | D |
+---+---+";

    // Parse the maze
    let maze = maze_str.parse::<Maze>().unwrap();

    // Print the parsed maze structure
    println!("{:?}", maze);

    // Access individual cells
    let cell_a = &maze.cells[0][0];
    println!("Cell A: {:?}", cell_a);

    let cell_b = &maze.cells[0][1];
    println!("Cell B: {:?}", cell_b);
}
```

## Maze Formats

### Small Maze Format:

```text
+   +   +
| A | D |
+   +   +
| B   C |
+---+---+
```

### Large Maze Format:

```text
+   ++   +
| A || D |
+   ++   +
+   ++   +
| B    C |
+---++---+
```
