use std::io;
use std::fmt;

/// Idea: a tic tac toe game in the shell
///
/// Breakdown:
///     1. drawing the sudoku
///     2. getting user input
///     3. evaluating if one player won


struct Board {
    data: [[char; 3]; 3],
    turn: i32,
}

impl Board {
    fn new() -> Board{
        return Board{
            data: [[' '; 3]; 3],
            turn: 1,
        };
    }

    fn toggle_turn(&mut self) {
        if self.turn == 1 {
            self.turn = 2;
        } else if self.turn == 2 {
            self.turn = 1;
        }
    }

    fn get_user_input(&mut self) {
        println!("Input your action: (format: RowColumn example: 02)");

        let mut i: usize;
        let mut j: usize;
        loop {
            let mut char1: char;
            let mut char2: char;
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Could not read line!");
        
                let trimmed_input = input.trim();
                let valid_chars = ['0', '1', '2'];
        
                if (trimmed_input.len() == 2) & trimmed_input.is_ascii(){
                    char1 = trimmed_input.chars().nth(0).unwrap();
                    char2 = trimmed_input.chars().nth(1).unwrap();
                    if valid_chars.contains(&char1) & valid_chars.contains(&char2){break;}
                }
        
                println!("Wrong input!");
            }
    
            i = char1 as usize - 48;
            j = char2 as usize - 48;
    
            if self.data[i][j] == ' ' {break;}
            
            println!("Field already occupied!");
        }
     
        if self.turn == 1 {
            self.data[i][j] = 'X';
        } else if self.turn == 2 {
            self.data[i][j] = 'O';
        }
    }

    fn check_finished(&self) -> bool{
        fn is_3_equals(chars: &[char; 3]) -> bool {
            if (*chars == ['X'; 3]) | (*chars == ['O'; 3]) {
                return true;
            }
            return false;
        }
    
        //check for rows
        fn check_rows(data: &[[char; 3]; 3]) -> bool {
            for row in 0..3 {
                if is_3_equals(&data[row]) {return true;}
            }
            return false;
        }

        //check for columns
        fn check_columns(data: &[[char; 3]; 3]) -> bool {
            for column in 0..3 {
                let mut chars = [' '; 3];
                for row in 0..3 {
                    chars[row] = data[row][column];
                }
            
                if is_3_equals(&chars) {return true;}
            }
            return false;
        }

        //check for diagonals
        fn check_diagonals(data: &[[char; 3]; 3]) -> bool {
            let chars = [data[0][0], data[1][1], data[2][2]];
            if is_3_equals(&chars) {return true;}
            let chars = [data[0][2], data[1][1], data[2][0]];
            if is_3_equals(&chars) {return true;}

            return false;
        }

        //check for all occupied
        fn is_full(data: &[[char; 3]; 3]) -> bool {
            let mut is_full = true;
            for row in data {
                for item in row {
                    if *item == ' ' {is_full = false;}
                }
            }
            if is_full {return true;}

            return false;
        }

        if check_rows(&self.data) | check_columns(&self.data) | check_diagonals(&self.data){
            println!("{esc}c", esc = 27 as char);
            println!("{}", self);
            println!("Player{} won!!!", self.turn);
            return true;
        } else if is_full(&self.data) {
            println!("{esc}c", esc = 27 as char);
            println!("{}", self);
            println!("Draw...");
            return true;
        }
        return false;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f,
            "-------\n|{}|{}|{}|\n-------\n|{}|{}|{}|\n-------\n|{}|{}|{}|\n-------\n",
            self.data[0][0], self.data[0][1], self.data[0][2],
            self.data[1][0], self.data[1][1], self.data[1][2],
            self.data[2][0], self.data[2][1], self.data[2][2],
        );
    }
}

fn main() {

    let mut board = Board::new();

    //game loop
    loop {
        print!("{esc}c", esc = 27 as char);
        println!("{}", board);
        board.get_user_input();
        if board.check_finished() {break;}
        board.toggle_turn();
    }
    let mut fake_input = String::new();
    io::stdin().read_line(&mut fake_input).expect("Could not read line!");
}
