use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};


#[cfg(test)]
mod manhattan_tests {
    use super::{manhattan_distance};


    #[test]
    fn test_manhattan_one() {
        assert_eq!(
            6, manhattan_distance(&vec!["R8", "U5", "L5", "D3"], &vec!["U7", "R6", "D4", "L4"])
        );
    }

    #[test]
    fn test_manhattan_two() {
        assert_eq!(
            6, manhattan_distance(&vec!["R8", "U5", "L5", "D3"], &vec!["U7", "R6", "D4", "L4"])
        );
    }

    #[test]
    fn test_manhattan_three() {
        assert_eq!(
            159, manhattan_distance(&vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
                                    &vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"])
        );
    }

    #[test]
    fn test_manhattan_four() {
        assert_eq!(
            135, manhattan_distance(&vec!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"],
                                    &vec!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"])
        );
    }
}


pub fn run_1() ->  Option<i64> {
    Some(run_program("input/day_three.txt").unwrap())
}


fn run_program(filename: &str) -> Option<i64> {
    Some(1)
}


fn read_wire_lengths(filename: &str) -> Result<Vec<i64>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut opcodes = Vec::new();

    for line in reader.lines() {
        for opcode in line?.split(',') {
            opcodes.push(opcode
                .trim()
                .parse::<i64>()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
        }
    }

    Ok(opcodes)
}


fn manhattan_distance(wire_1: &Vec<&str>, wire_2: &Vec<&str>) -> i64 {
    0
}