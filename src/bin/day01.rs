use std::fs;

pub fn main() {
    let input = fs::read_to_string("inputs/01.txt").unwrap();
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in input.lines() {
        let parts: Vec<i32> = line.split("   ").map(|part| {part.parse::<i32>().unwrap()}).collect();
        left.push(parts[0]);
        right.push(parts[1]);
    }

    left.sort();
    right.sort();

    let mut distance = 0;
    let mut similarity = 0;

    for (index, item) in left.iter().enumerate() {
        distance = distance + (item - right[index]).abs();
        similarity = similarity + (
            *item as usize * right.iter().filter(|&n| n == &left[index]).count()
        );
    }

    fs::write(
        "outputs/01.txt",
        format!("Part 1: {}\nPart 2: {}", distance, similarity)
    ).unwrap();
}
