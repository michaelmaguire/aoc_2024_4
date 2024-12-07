use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;



fn initialize_vector_of_checker_matrices() -> Vec<Vec<Vec<char>>> {

    let mut checker_matrices : Vec<Vec<Vec<char>>> = Vec::new();

    checker_matrices.push( vec![vec!['X', 'M', 'A', 'S'], vec!['\0','\0','\0','\0'], vec!['\0','\0','\0','\0'], vec!['\0','\0','\0','\0']] );
    checker_matrices.push( vec![vec!['\0','\0','\0','\0'], vec!['X', 'M', 'A', 'S'], vec!['\0','\0','\0','\0'], vec!['\0','\0','\0','\0']] );
    checker_matrices.push( vec![vec!['\0','\0','\0','\0'], vec!['\0','\0','\0','\0'], vec!['X', 'M', 'A', 'S'], vec!['\0','\0','\0','\0']] );
    checker_matrices.push( vec![vec!['\0','\0','\0','\0'], vec!['\0','\0','\0','\0'], vec!['\0','\0','\0','\0'], vec!['X', 'M', 'A', 'S']] );

    checker_matrices.push( vec![vec!['S', 'A', 'M', 'X'], vec!['\0','\0','\0','\0'], vec!['\0','\0','\0','\0'], vec!['\0','\0','\0','\0']] );
    checker_matrices.push( vec![vec!['\0','\0','\0','\0'], vec!['S', 'A', 'M', 'X'], vec!['\0','\0','\0','\0'], vec!['\0','\0','\0','\0']] );
    checker_matrices.push( vec![vec!['\0','\0','\0','\0'], vec!['\0','\0','\0','\0'], vec!['S', 'A', 'M', 'X'], vec!['\0','\0','\0','\0']] );
    checker_matrices.push( vec![vec!['\0','\0','\0','\0'], vec!['\0','\0','\0','\0'], vec!['\0','\0','\0','\0'], vec!['S', 'A', 'M', 'X']] );

    checker_matrices.push( vec![vec!['X','\0','\0','\0'], vec!['\0','X','\0','\0'], vec!['\0','\0','X','\0'], vec!['\0','\0','\0','X']] );
    checker_matrices.push( vec![vec!['M','\0','\0','\0'], vec!['\0','M','\0','\0'], vec!['\0','\0','M','\0'], vec!['\0','\0','\0','M']] );
    checker_matrices.push( vec![vec!['A','\0','\0','\0'], vec!['\0','S','\0','\0'], vec!['\0','\0','A','\0'], vec!['\0','\0','\0','A']] );
    checker_matrices.push( vec![vec!['S','\0','\0','\0'], vec!['\0','A','\0','\0'], vec!['\0','\0','S','\0'], vec!['\0','\0','\0','S']] );

    checker_matrices.push( vec![vec!['S','\0','\0','\0'], vec!['\0','S','\0','\0'], vec!['\0','\0','S','\0'], vec!['\0','\0','\0','S']] );
    checker_matrices.push( vec![vec!['A','\0','\0','\0'], vec!['\0','A','\0','\0'], vec!['\0','\0','A','\0'], vec!['\0','\0','\0','A']] );
    checker_matrices.push( vec![vec!['M','\0','\0','\0'], vec!['\0','M','\0','\0'], vec!['\0','\0','M','\0'], vec!['\0','\0','\0','M']] );
    checker_matrices.push( vec![vec!['X','\0','\0','\0'], vec!['\0','X','\0','\0'], vec!['\0','\0','X','\0'], vec!['\0','\0','\0','X']] );

    checker_matrices.push( vec![vec!['X','\0','\0','\0'], vec!['\0', 'M','\0','\0'], vec!['\0','\0', 'A','\0'], vec!['\0','\0','\0','S']] );
    checker_matrices.push( vec![vec!['S','\0','\0','\0'], vec!['\0', 'A','\0','\0'], vec!['\0','\0', 'M','\0'], vec!['\0','\0','\0','X']] );
    checker_matrices.push( vec![vec!['\0','\0','\0', 'X'], vec!['\0','\0', 'M','\0'], vec!['\0', 'A','\0','\0'], vec!['S','\0','\0','\0']] );
    checker_matrices.push( vec![vec!['\0','\0','\0', 'S'], vec!['\0','\0', 'A','\0'], vec!['\0', 'M','\0','\0'], vec!['X','\0','\0','\0']] );

    return checker_matrices;
}





fn check_all_at_position( i: usize, j: usize, input_matrix: &Vec<Vec<char>>, checker_matrices: &Vec<Vec<Vec<char>>>) -> u64 {

    fn check_single_checker(i: usize, j: usize, input_matrix: &Vec<Vec<char>>, checker_matrix: &Vec<Vec<char>> ) -> bool {
        for locali in 0..checker_matrix.len() {
            for localj in 0..checker_matrix.len() {
                let check_value = checker_matrix[locali][localj];
                if check_value != '\0' {
                    let input_value = input_matrix[i+locali][j+localj];
                    if check_value != input_value {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    let mut count: u64 = 0;

    for checker in checker_matrices {
        if check_single_checker(i,j,input_matrix,checker) {
            count += 1;
        }
    }

    return count;
}


fn main() {
    println!("Hello, aoc_2024_4!");

    if let Ok(lines) = read_lines("./src/input.txt") {

        let mut input_matrix: Vec<Vec<char>> = Vec::new();

        // Consumes the iterator, returns an ( Optional) String
        for line in lines.flatten() {
            let characters:Vec<char> = line.chars().collect();
            input_matrix.push(characters);
        }

        let height = input_matrix.len();
        println!("height {height}");

        // We assume every row has the same number of columns.
        let width = input_matrix[0].len();
        println!("array width {width}");
        
        for i in 0..height {
            let row = &input_matrix[i];
            for j in 0..row.len() {
                let val = input_matrix[i][j];
                print!("{val} ");
            }
            println!();
        }        

        let checker_matrices = initialize_vector_of_checker_matrices();

        // We assume every row has the same number of columns.
        let check_width = checker_matrices[0][0].len();
        println!("check_width {check_width}");

        let mut count_found: u64 = 0;

        for i in 0..(height-check_width+1) {
            for j in 0..(width-check_width+1) {
                count_found += check_all_at_position(i,j,&input_matrix,&checker_matrices);
            }
        }


        println!("count_found {count_found}");

    } else {
        if let Ok(path) = env::current_dir() {
            println!("Error reading lines, the current directory is {}", path.display());
        } else {
            println!("Error reading lines, and can't print the current directory");

        }
    }
}

// Thanks to https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}