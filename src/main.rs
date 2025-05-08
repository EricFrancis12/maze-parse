pub mod cell;
pub mod errors;
pub mod maze;
#[cfg(test)]
pub mod maze_lg_test;
#[cfg(test)]
pub mod maze_sm_test;
pub mod parser;

use crate::maze::Maze;

fn main() {
    let sm_maze = Maze::new_from_file("my_maze_sm").unwrap();
    println!("--- START SMALL ---\n{:?}\n---  END SMALL  ---\n", &sm_maze);

    let lg_maze = Maze::new_from_file("my_maze_lg").unwrap();
    println!("--- START LARGE ---\n{:?}\n---  END LARGE  ---\n", &lg_maze);
}
