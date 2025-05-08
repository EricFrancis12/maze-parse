use crate::{cell::Cell, maze::Maze};

#[test]
fn test_parse_sm_maze() {
    let input = "\
+---+
| A |
+---+";

    let expected = Maze {
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
    };

    let parsed = Maze::parse_sm(input).unwrap();
    assert_eq!(parsed, expected);
}
