const SIZE: usize = 4;
fn neighbors(board: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut count = 0;

    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 { continue; } // Skip The Target Cell

            // Adjust For Wrap Around Using Modulo %
            let nx = (x + i + SIZE - 1) % SIZE;
            let ny = (y + j + SIZE - 1) % SIZE;

            if board[nx][ny] {
                count += 1;
            }
        }
    }

    count // return count
}

fn evolve(board: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_board = vec![vec![false; SIZE]; SIZE];
    for x in 0..SIZE {
        for y in 0..SIZE {
            let cell = board[x][y];
            let live_neighbors = neighbors(&board, x, y);

            new_board[x][y] = match (cell, live_neighbors) {
                (true, 2) | (_, 3) => true,
                _ => false,
            };
        }
    }

    new_board
}


fn main() {
    println!("Hello World");
    let mut board = vec![vec![false; 4]; 4]; // Using Boolean Expressions to Represent Spaces With Cells
    let initial_cells = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
    for &(x, y) in &initial_cells { // Inumerating Through Initial Cells to Find the Position For Initial Board
        board[x][y] = true;
    }
    println!("{:?}", board);
    println!("{}", neighbors(&board, 2, 1));
    for _ in 0..10 {
        board = evolve(board);
    }
    println!("{:?}", board)
}