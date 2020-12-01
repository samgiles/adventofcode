use std::collections::BinaryHeap;
use std::io::{self, BufRead, BufReader};
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
