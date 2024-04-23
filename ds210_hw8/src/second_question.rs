const SIZE: usize = 4; // Size of the Row and Column
fn neighbors(board: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut count = 0; // Count For Neighbors

    for i in 0..3 { // 0, 1, 2
        for j in 0..3 { // 0, 1, 2 
            if i == 1 && j == 1 { continue; } // Skip The Target Cell
            // Generates a 3x3 Grid With the Target Point in the Middle --> Ignores Target at (1, 1)
            // Adjust For Wrap Around Using Modulo %
            let nx = (x + i + SIZE - 1) % SIZE;
            let ny = (y + j + SIZE - 1) % SIZE;

            if board[nx][ny] {
                count += 1;
            }
        }
    }

    count // Return Count
}

fn evolve(board: Vec<Vec<bool>>) -> Vec<Vec<bool>> { // Evolves the Board 1 Step
    let mut new_board = vec![vec![false; SIZE]; SIZE];
    for x in 0..SIZE {
        for y in 0..SIZE {
            let cell = board[x][y]; // Reference to Cell at (X, Y)
            let live_neighbors = neighbors(&board, x, y); // Current Live Neighbors of Point at (x, y)

            new_board[x][y] = match (cell, live_neighbors) { // Match is Used to Compare New Board Point (x, y) and the Requirements for Cell Death, Cell Birth, and Continued Cell Life
                (true, 2) | (_, 3) => true,
                _ => false,
            };
        }
    }

    new_board
}

#[cfg(test)]
mod tests {
    use super::*; // Import Functions From Outer Module

    #[test] // Necessary to Run Test On Code
    fn no_neighbors() { // Test to Ensure That A Cell With No Neighbors is Correct
        let board = vec![vec![false; SIZE]; SIZE];
        // Choose a cell that we know has no neighbors
        assert_eq!(neighbors(&board, 1, 1), 0); // Point (1, 1) Does Not Have Any Neighbors
    }
}
fn main() {
    let mut board = vec![vec![false; 4]; 4]; // Using Boolean Expressions to Represent Spaces With Cells
    let initial_cells = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)]; // Initial Points
    for &(x, y) in &initial_cells { // Inumerating Through Initial Cells to Find the Position For Initial Board
        board[x][y] = true;
    }
    println!("{:?}", board); // Initial Board : Vector of Vectors
    println!("{}", neighbors(&board, 2, 1)); // Test of Neighbors 
    for _ in 0..10 { // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 --> Uses For Loop to Iterate Through Board 10 Times
        board = evolve(board);
    }
}