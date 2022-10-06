
pub fn main() {
    let side = 4;

    let res = solve_n_queens(side);
    println!("{:?}", res)
}


pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    fn render(board: &Vec<usize>, size: usize) -> Vec<String> {
        board.iter().map(|&i| { ".".repeat(i) + "Q" + &".".repeat(size - i - 1) }).collect()
    }
    fn is_valid(board: &[usize], queen_pos: (usize, usize)) -> bool {
        for i in 0..queen_pos.0 {
            if queen_pos.1 == board[i] || (queen_pos.0 as i32 - i as i32).abs() == (queen_pos.1 as i32 - board[i] as i32).abs() {
                return false;
            }
        }
        true
    }
    fn back_track(size: usize, row: usize, board: &mut Vec<usize>, res: &mut Vec<Vec<String>>) {
        if row >= size {
            res.push(render(board, size));
            return;
        }
        for col in 0..size {
            if is_valid(&board[..], (row, col)) {
                board[row] = col;
                back_track(size, row + 1, board, res);
            }
        }
    }
    let mut board = vec![0; n as usize];
    let mut res = vec![];
    back_track(n as usize, 0, &mut board, &mut res);
    res
}



fn all_direction(queens: &mut Vec<Vec<i32>>, y: usize, x: usize) {
    if queens[y][x] == 1 { return }

    queens[y][x] = 0;

    if y > 0 { all_direction(queens, y - 1, x); };    // top
    if y <= queens.len() - 1 { all_direction(queens, y + 1, x); };    // bottom
    if x > 0 { all_direction(queens, y, x - 1); };    // left
    if x <= queens[0].len() - 1 { all_direction(queens, y, x + 1); }; // right

    if y > 0 && x > 0 {
        all_direction(queens, y - 1, x - 1)   // top_left
    };
    if y <= queens.len() - 1 && x <= queens[0].len() - 1 {
        all_direction(queens, y + 1, x + 1)   // bottom_right
    };
    if y > 0 && x <= queens[0].len() - 1 {
        all_direction(queens, y - 1, x + 1);  // top_right
    };
    if y <= queens.len() - 1 && x > 0 {
        all_direction(queens, y + 1, x - 1);   // bottom_left
    };

}