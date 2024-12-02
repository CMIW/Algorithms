# Valid Sudoku

You are given a ``` 9 x 9 ``` Sudoku board. A Sudoku board is valid if the following rules are followed:
1. Each row must contain the digits ``` 1-9 ``` without duplicates.
2. Each column must contain the digits ``` 1-9 ``` without duplicates.
3. Each of the nine ``` 3 x 3 ``` sub-boxes of the grid must contain the digits ``` 1-9 ``` without duplicates.

Return ``` true ``` if the Sudoku board is valid, otherwise return ``` false ```

Note: A board does not need to be full or be solvable to be valid.

```
FUNCTION valid_sudoku(board):
    // Initialize bitmasks for rows, columns, and 3x3 sub-grids
    rows = ARRAY[9] OF 0
    cols = ARRAY[9] OF 0
    subs = ARRAY[9] OF 0

    // Iterate over each cell in the 9x9 board
    FOR i FROM 0 TO 8:
        FOR j FROM 0 TO 8:
            // Try to convert the cell value to an integer
            value = PARSE_INT(board[i][j])

            IF value IS NOT VALID:
                CONTINUE // Skip empty or invalid cells

            value_index = value - 1
            shift = 1 << value_index // Create bitmask for the value

            // Check for duplicates in the current row
            IF (rows[i] & shift) == 0:
                rows[i] = rows[i] | shift // Mark value as seen in the row
            ELSE:
                RETURN false // Duplicate found in the row

            // Check for duplicates in the current column
            IF (cols[j] & shift) == 0:
                cols[j] = cols[j] | shift // Mark value as seen in the column
            ELSE:
                RETURN false // Duplicate found in the column

            // Calculate sub-grid index
            sub_index = (i / 3) * 3 + (j / 3)

            // Check for duplicates in the 3x3 sub-grid
            IF (subs[sub_index] & shift) == 0:
                subs[sub_index] = subs[sub_index] | shift // Mark value as seen in sub-grid
            ELSE:
                RETURN false // Duplicate found in the sub-grid

    // If no duplicates were found, the board is valid
    RETURN true
```
