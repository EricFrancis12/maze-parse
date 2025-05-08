use crate::{cell::Cell, maze::Maze};

#[test]
fn test_parse_valid_lg_mazes() {
    let tests = vec![
        (
            " --- \n| A |\n --- ",
            Maze {
                cells: vec![vec![Cell {
                    wall_top: true,
                    wall_bottom: true,
                    wall_left: true,
                    wall_right: true,
                    corner_top_left: false,
                    corner_top_right: false,
                    corner_bottom_left: false,
                    corner_bottom_right: false,
                    inner_text: String::from(" A "),
                }]],
            },
        ),
        (
            "\
+   +
  A  
+   +",
            Maze {
                cells: vec![vec![Cell {
                    wall_top: false,
                    wall_bottom: false,
                    wall_left: false,
                    wall_right: false,
                    corner_top_left: true,
                    corner_top_right: true,
                    corner_bottom_left: true,
                    corner_bottom_right: true,
                    inner_text: String::from(" A "),
                }]],
            },
        ),
        (
            "\
+---+
  A |
    +",
            Maze {
                cells: vec![vec![Cell {
                    wall_top: true,
                    wall_bottom: false,
                    wall_left: false,
                    wall_right: true,
                    corner_top_left: true,
                    corner_top_right: true,
                    corner_bottom_left: false,
                    corner_bottom_right: true,
                    inner_text: String::from(" A "),
                }]],
            },
        ),
        (
            "\
+   +
  A |
 --- ",
            Maze {
                cells: vec![vec![Cell {
                    wall_top: false,
                    wall_bottom: true,
                    wall_left: false,
                    wall_right: true,
                    corner_top_left: true,
                    corner_top_right: true,
                    corner_bottom_left: false,
                    corner_bottom_right: false,
                    inner_text: String::from(" A "),
                }]],
            },
        ),
        (
            "\
+---+
| A |
+---+",
            Maze {
                cells: vec![vec![Cell {
                    wall_top: true,
                    wall_bottom: true,
                    wall_left: true,
                    wall_right: true,
                    corner_top_left: true,
                    corner_top_right: true,
                    corner_bottom_left: true,
                    corner_bottom_right: true,
                    inner_text: String::from(" A "),
                }]],
            },
        ),
        (
            "\
+---++---+
| A || B |
+---++---+",
            Maze {
                cells: vec![vec![
                    Cell {
                        wall_top: true,
                        wall_bottom: true,
                        wall_left: true,
                        wall_right: true,
                        corner_top_left: true,
                        corner_top_right: true,
                        corner_bottom_left: true,
                        corner_bottom_right: true,
                        inner_text: String::from(" A "),
                    },
                    Cell {
                        wall_top: true,
                        wall_bottom: true,
                        wall_left: true,
                        wall_right: true,
                        corner_top_left: true,
                        corner_top_right: true,
                        corner_bottom_left: true,
                        corner_bottom_right: true,
                        inner_text: String::from(" B "),
                    },
                ]],
            },
        ),
        (
            "\
+   ++---+
  A || B |
+---++---+
+---++---+
| C || D  
+---++    ",
            Maze {
                cells: vec![
                    vec![
                        Cell {
                            wall_top: false,
                            wall_bottom: true,
                            wall_left: false,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: true,
                            inner_text: String::from(" A "),
                        },
                        Cell {
                            wall_top: true,
                            wall_bottom: true,
                            wall_left: true,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: true,
                            inner_text: String::from(" B "),
                        },
                    ],
                    vec![
                        Cell {
                            wall_top: true,
                            wall_bottom: true,
                            wall_left: true,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: true,
                            inner_text: String::from(" C "),
                        },
                        Cell {
                            wall_top: true,
                            wall_bottom: false,
                            wall_left: true,
                            wall_right: false,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: false,
                            inner_text: String::from(" D "),
                        },
                    ],
                ],
            },
        ),
        (
            "\
+   ++---+
  A || B |
+    +---+
+---++---+
| C || D  
+---++    ",
            Maze {
                cells: vec![
                    vec![
                        Cell {
                            wall_top: false,
                            wall_bottom: false,
                            wall_left: false,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: false,
                            inner_text: String::from(" A "),
                        },
                        Cell {
                            wall_top: true,
                            wall_bottom: true,
                            wall_left: true,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: true,
                            inner_text: String::from(" B "),
                        },
                    ],
                    vec![
                        Cell {
                            wall_top: true,
                            wall_bottom: true,
                            wall_left: true,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: true,
                            inner_text: String::from(" C "),
                        },
                        Cell {
                            wall_top: true,
                            wall_bottom: false,
                            wall_left: true,
                            wall_right: false,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: false,
                            inner_text: String::from(" D "),
                        },
                    ],
                ],
            },
        ),
        (
            "\
+---++---+
| A || B |
+---++---+
+---+ --- 
| C |  D  
+---++    ",
            Maze {
                cells: vec![
                    vec![
                        Cell {
                            wall_top: true,
                            wall_bottom: true,
                            wall_left: true,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: true,
                            inner_text: String::from(" A "),
                        },
                        Cell {
                            wall_top: true,
                            wall_bottom: true,
                            wall_left: true,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: true,
                            inner_text: String::from(" B "),
                        },
                    ],
                    vec![
                        Cell {
                            wall_top: true,
                            wall_bottom: true,
                            wall_left: true,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: true,
                            inner_text: String::from(" C "),
                        },
                        Cell {
                            wall_top: true,
                            wall_bottom: false,
                            wall_left: false,
                            wall_right: false,
                            corner_top_left: false,
                            corner_top_right: false,
                            corner_bottom_left: true,
                            corner_bottom_right: false,
                            inner_text: String::from(" D "),
                        },
                    ],
                ],
            },
        ),
        (
            "\
+---++---+
| A || B |
+---++---+
+---++---+
| C || D  
+---++    ",
            Maze {
                cells: vec![
                    vec![
                        Cell {
                            wall_top: true,
                            wall_bottom: true,
                            wall_left: true,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: true,
                            inner_text: String::from(" A "),
                        },
                        Cell {
                            wall_top: true,
                            wall_bottom: true,
                            wall_left: true,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: true,
                            inner_text: String::from(" B "),
                        },
                    ],
                    vec![
                        Cell {
                            wall_top: true,
                            wall_bottom: true,
                            wall_left: true,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: true,
                            inner_text: String::from(" C "),
                        },
                        Cell {
                            wall_top: true,
                            wall_bottom: false,
                            wall_left: true,
                            wall_right: false,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: false,
                            inner_text: String::from(" D "),
                        },
                    ],
                ],
            },
        ),
        (
            "\
+---++---+
| A || B |
+---++---+
+---++---+
| C || D |
+---++---+",
            Maze {
                cells: vec![
                    vec![
                        Cell {
                            wall_top: true,
                            wall_bottom: true,
                            wall_left: true,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: true,
                            inner_text: String::from(" A "),
                        },
                        Cell {
                            wall_top: true,
                            wall_bottom: true,
                            wall_left: true,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: true,
                            inner_text: String::from(" B "),
                        },
                    ],
                    vec![
                        Cell {
                            wall_top: true,
                            wall_bottom: true,
                            wall_left: true,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: true,
                            inner_text: String::from(" C "),
                        },
                        Cell {
                            wall_top: true,
                            wall_bottom: true,
                            wall_left: true,
                            wall_right: true,
                            corner_top_left: true,
                            corner_top_right: true,
                            corner_bottom_left: true,
                            corner_bottom_right: true,
                            inner_text: String::from(" D "),
                        },
                    ],
                ],
            },
        ),
    ];

    for (input, expected) in tests {
        let parsed = Maze::parse_lg(input).unwrap();
        assert_eq!(parsed, expected);
    }
}
