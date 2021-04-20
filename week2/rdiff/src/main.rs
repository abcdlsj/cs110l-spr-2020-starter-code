use grid::Grid; // For lcs()
use std::{cmp::{self, max}, env};
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()
use std::process;

pub mod grid;

/// Reads the file at the supplied path, and returns a vector of strings.
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let mut lines: Vec<String> = Vec::new();
    let f = File::open(filename)?;
    for line in io::BufReader::new(f).lines() {
        let line_str = line?;
        lines.push(line_str);
    }

    Ok(lines)
    // Be sure to delete the #[allow(unused)] line above
}

fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    // Note: Feel free to use unwrap() in this code, as long as you're basically certain it'll
    // never happen. Conceptually, unwrap() is justified here, because there's not really any error
    // condition you're watching out for (i.e. as long as your code is written correctly, nothing
    // external can go wrong that we would want to handle in higher-level functions). The unwrap()
    // calls act like having asserts in C code, i.e. as guards against programming error.
    let (X, Y) = (seq1, seq2);
    let (m, n) = (X.len(), Y.len());
    let mut grid = grid::Grid::new(m + 1, n + 1);

    for i in 0..m + 1 {
        grid.set(i, 0, 0).unwrap();
    }

    for j in 0..n + 1 {
        grid.set(0, j, 0).unwrap();
    }

    for i in 0..m {
        for j in 0..n {
            if X[i] == Y[j] {
                grid.set(i + 1, j + 1, grid.get(i, j).unwrap() + 1);
            } else {
                grid.set(i + 1, j + 1, cmp::max(grid.get(i, j + 1).unwrap(), grid.get(i + 1, j).unwrap()));
            }
        }
    }
    grid
    // Be sure to delete the #[allow(unused)] line above
}

fn print_diff(C: &Grid, X: &Vec<String>, Y: &Vec<String>, i: usize, j: usize) {
    if i > 0 && j > 0 && X[i-1] == Y[j-1] {
        print_diff(C, X, Y, i-1, j-1);
        println!("  {}", X[i-1]);
    } else if j > 0 && (i == 0 || C.get(i,j-1) >= C.get(i-1,j)) {
        print_diff(C, X, Y, i, j-1);
        println!("> {}", Y[j-1]);
    } else if i > 0 && (j == 0 || C.get(i,j-1) < C.get(i-1,j)) {
        print_diff(C, X, Y, i-1, j);
        println!("< {}", X[i-1]);
    } else {
        println!("");
    }
    // Be sure to delete the #[allow(unused)] line above
}

#[allow(unused)] // TODO: delete this line when you implement this function
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    unimplemented!();
    // Be sure to delete the #[allow(unused)] line above
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }

    #[test]
    fn test_lcs() {
        let mut expected = Grid::new(5, 4);
        expected.set(1, 1, 1).unwrap();
        expected.set(1, 2, 1).unwrap();
        expected.set(1, 3, 1).unwrap();
        expected.set(2, 1, 1).unwrap();
        expected.set(2, 2, 1).unwrap();
        expected.set(2, 3, 2).unwrap();
        expected.set(3, 1, 1).unwrap();
        expected.set(3, 2, 1).unwrap();
        expected.set(3, 3, 2).unwrap();
        expected.set(4, 1, 1).unwrap();
        expected.set(4, 2, 2).unwrap();
        expected.set(4, 3, 2).unwrap();

        println!("Expected:");
        expected.display();
        let result = lcs(
            &"abcd".chars().map(|c| c.to_string()).collect(),
            &"adb".chars().map(|c| c.to_string()).collect(),
        );
        println!("Got:");
        result.display();
        assert_eq!(result.size(), expected.size());
        for row in 0..expected.size().0 {
            for col in 0..expected.size().1 {
                assert_eq!(result.get(row, col), expected.get(row, col));
            }
        }
    }
}
