use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_input_as_list_of_sorted_integers<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<i64>, anyhow::Error> {
    // We can use a heap to efficiently build a sorted list as we read

    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);

    let mut heap = BinaryHeap::new();
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        let number = line.parse::<i64>()?;
        heap.push(number);
    }

    Ok(heap.into_sorted_vec())
}

#[derive(Debug)]
pub struct PasswordRecord {
    pub lowest: i64,
    pub highest: i64,
    pub character: char,
    pub password: String,
}

pub fn read_password_input<P: AsRef<Path>>(path: P) -> impl Iterator<Item = PasswordRecord> {
    let file = std::fs::File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| {
        let line = line.unwrap();
        let mut whitespace = line.split_whitespace();

        let range = whitespace.next().unwrap();
        let character = whitespace.next().unwrap();
        let password = whitespace.next().unwrap();

        let (lowest, highest) = parse_range(range);

        PasswordRecord {
            lowest,
            highest,
            character: character.as_bytes()[0] as char,
            password: password.to_string(),
        }
    })
}

fn parse_range(range: &str) -> (i64, i64) {
    let mut range = range.split('-');

    let from = range.next().unwrap();
    let to = range.next().unwrap();

    (from.parse::<i64>().unwrap(), to.parse::<i64>().unwrap())
}

pub fn read_ski_slope_map<P: AsRef<Path>>(path: P) -> SkiSlope {
    let file = std::fs::File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut map = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut row = Vec::new();
        for character in line.as_bytes() {
            let character = *character as char;

            let space = match character {
                '.' => SkiSlopeSpace::Free,
                '#' => SkiSlopeSpace::Tree,
                _ => panic!("unknown space: {:?}", character),
            };

            row.push(space);
        }
        map.push(row);
    }

    SkiSlope::new(map)
}

pub enum SkiSlopeSpace {
    Free,
    Tree,
}

pub struct SkiSlope {
    map: Vec<Vec<SkiSlopeSpace>>,
    pub width: usize,
    pub height: usize,
}

impl SkiSlope {
    pub fn new(map: Vec<Vec<SkiSlopeSpace>>) -> Self {
        let height = map.len();
        let width = map.get(0).unwrap().len();
        SkiSlope { map, height, width }
    }

    pub fn get_position(&self, x: usize, y: usize) -> &SkiSlopeSpace {
        let row = self.map.get(y).unwrap();
        row.get(x).unwrap()
    }

    pub fn get_position_wrapping(&self, x: usize, y: usize) -> &SkiSlopeSpace {
        let pos_x = x % self.width;
        // Only wrap the X axis
        self.get_position(pos_x, y)
    }
}
