fn main() {
    //Sudoku Grid to be Solved could be replaced by method call that retrieves from a file
    let mut sudoku:[[usize; 9]; 9] = [
        [3,0,6,5,0,8,4,0,0],
        [5,2,0,0,0,0,0,0,0],
        [0,8,7,0,0,0,0,3,1],
        [0,0,3,0,1,0,0,8,0],
        [9,0,0,8,6,3,0,0,5],
        [0,5,0,0,9,0,6,0,0],
        [1,3,0,0,0,0,2,5,0],
        [0,0,0,0,0,0,0,7,4],
        [0,0,5,2,0,6,3,0,0],
    ];
    
    //Attempt to solve as any sudoku board is solveable unless already having broken a rule
    if solve_sudoku(&mut sudoku){
        print_sudoku(sudoku);
    }
    else{
        //So that there is no silent failure if the board is unsolveable
        println!("No Solution");
    }
}

fn check_row(grid: [[usize; 9]; 9], num: usize, row: usize) -> bool{
    for i in 0..9{
        //Iterate through whole row looking to match
        if grid[row][i] == num{
           return false
        }
    }
    true
}

fn check_column(grid: [[usize; 9]; 9], num: usize, col: usize) -> bool{
    for i in 0..9{
        //Iterate through whole column looking to match
        if grid[i][col] == num{
            return false
        }
    }
    true
}

fn check_quadrant(grid: [[usize; 9]; 9], num: usize, start_row: usize, start_col: usize) -> bool{
    for i in 0..3{
        for j in 0..3{
            //Iterate through whole quadrant looking to match
            if grid[i + start_row][j + start_col] == num{
                return false
            }
        }
    }
    true
}

fn check_location(grid: [[usize; 9]; 9], row: usize, col: usize, num: usize) -> bool{
    //To make code more readable when needing to check
    check_row(grid, num, row) && check_column(grid, num, col) && check_quadrant(grid, num, row - row % 3, col - col % 3)
}

fn find_unfilled(grid: [[usize; 9]; 9]) -> Unfilled{
    let mut location = Unfilled{
        found: false,
        row: 0,
        col: 0,
    };

    for i in 0..9{
        for j in 0..9{
            //Iterate whole board looking for 0 as a blank space
            if grid[i][j] == 0{
                location.found = true;
                location.row = i;
                location.col = j;
                
                //Uses Struct to return bool on if one is found along with coords to it
                return location;
            }
        }
    }
    location
}

//Recursive Solve Method
fn solve_sudoku(grid: &mut[[usize; 9]; 9] ) -> bool {
    let location: Unfilled = find_unfilled(*grid);

    //If no more blank spaces then the board is solved
    if !location.found{
        true
    }
    else
    {
        //Get coordinates
        let row = location.row;
        let col = location.col;

        //Recursive check for values from 1 to 9 to fill in this place
        for num in 1..10{
            if check_location(*grid, row, col, num){
                grid[row][col] = num;

                if solve_sudoku(grid){
                    return true
                }
                else{
                    grid[row][col] = 0;
                }
            }
        }
        false
    }
}

//Prints out Sudoku Board
fn print_sudoku(grid:[[usize; 9]; 9]){
    let mut output: String = "".to_string();
    for i in 0..9{
        output = [output,"-------------------\n".to_string()].concat();
        for j in 0..9{
            output = [output,"|".to_string()].concat();
            output = [output,grid[i][j].to_string()].concat();
        }
        output = [output,"|\n".to_string()].concat();
    }
    output = [output,"-------------------\n".to_string()].concat();

    println!("{}", output);
}

struct Unfilled{
    found: bool,
    row: usize,
    col: usize,
}