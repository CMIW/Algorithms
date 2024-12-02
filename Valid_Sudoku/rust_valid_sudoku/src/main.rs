use std::str::FromStr;

/// Checks if a given 9x9 Sudoku board is valid.
fn valid_sudoku(board: &[[&str; 9]; 9]) -> bool {
    // Arrays to track seen numbers in rows, columns, and sub-grids.
    // Each element is a 9-bit `u16` where each bit represents a number 1-9.
    let mut rows: [u16; 9] = [0; 9];
    let mut cols: [u16; 9] = [0; 9];
    let mut subs: [u16; 9] = [0; 9];

    for i in 0..=8 {
        for j in 0..=8 {
            // Try to parse the current cell's value into an integer.
            match usize::from_str(board[i][j]) {
                Ok(value) => {
                    // Calculate the bit index for the current value (1-9).
                    let value_index = value - 1;
                    // Create a bitmask with a 1 at the corresponding bit position.
                    let shift = 1_u16 << value_index;

                    // Check the current row's bitmask for duplicates.
                    // Bit & to extract the bit counter
                    if (rows[i] & shift) == 0 {
                        // Update the row's bitmask to mark the number as seen.
                        // Bit | to store the value counter
                        rows[i] = rows[i] | shift;
                    } else {
                        return false;
                    }

                    // Check the current column's bitmask for duplicates.
                    if (cols[j] & shift) == 0 {
                        // Update the column's bitmask to mark the number as seen.
                        cols[j] = cols[j] | shift;
                    } else {
                        return false;
                    }

                    // Calculate the index of the 3x3 sub-grid.
                    let sub_index: usize = (i / 3) * 3 + (j / 3);
                    // Check the sub-grid's bitmask for duplicates.
                    if (subs[sub_index] & shift) == 0 {
                        // Update the sub-grid's bitmask to mark the number as seen.
                        subs[sub_index] = subs[sub_index] | shift;
                    } else {
                        return false;
                    }
                }
                // If parsing fails (e.g., empty cells), do nothing and continue.
                _ => {}
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_test() {
        let board = [
            ["1", "2", ".", ".", "3", ".", ".", ".", "."],
            ["4", ".", ".", "5", ".", ".", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", ".", "3"],
            ["5", ".", ".", ".", "6", ".", ".", ".", "4"],
            [".", ".", ".", "8", ".", "3", ".", ".", "5"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", ".", ".", ".", ".", ".", "2", ".", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "8"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];

        assert_eq!(true, valid_sudoku(&board));
    }

    #[test]
    fn invalid_row() {
        let board = [
            ["1", "2", ".", ".", "3", ".", ".", ".", "1"],
            ["4", ".", ".", "5", ".", ".", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", ".", "3"],
            ["5", ".", ".", ".", "6", ".", ".", ".", "4"],
            [".", ".", ".", "8", ".", "3", ".", ".", "5"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", ".", ".", ".", ".", ".", "2", ".", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "8"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];

        assert_eq!(false, valid_sudoku(&board));
    }

    #[test]
    fn invalid_col() {
        let board = [
            ["1", "2", ".", ".", "3", ".", ".", ".", "."],
            ["4", ".", ".", "5", ".", ".", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", ".", "3"],
            ["5", ".", ".", ".", "6", ".", ".", ".", "4"],
            [".", ".", ".", "8", ".", "3", ".", ".", "5"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", ".", ".", ".", ".", ".", "2", ".", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "8"],
            ["1", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];

        assert_eq!(false, valid_sudoku(&board));
    }

    #[test]
    fn invalid_sub() {
        let board = [
            ["1", "2", ".", ".", "3", ".", ".", ".", "."],
            ["4", ".", "1", "5", ".", ".", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", ".", "3"],
            ["5", ".", ".", ".", "6", ".", ".", ".", "4"],
            [".", ".", ".", "8", ".", "3", ".", ".", "5"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", ".", ".", ".", ".", ".", "2", ".", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "8"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];

        assert_eq!(false, valid_sudoku(&board));
    }
}
