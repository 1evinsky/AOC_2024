mod file_read;
mod task1;
mod task2;

use file_read::FileReader;
use task1::Task1;
use task2::Task2;

fn main() 
{
    let file_reader: FileReader = FileReader::new("src/input.txt");

    let patterns: Vec<String> = vec!["XMAS".to_string(), "SAMX".to_string()];

    let lines: Vec<String> = file_reader.read_by_lines().unwrap();

    let task1 = Task1::new(patterns, lines.clone());

    let patterns: Vec<String> = vec!["MAS".to_string(), "SAM".to_string()];

    let task2 = Task2::new(patterns, lines.clone());

    println!("Task 1 Answer: {}", task1.count());
    println!("Task 2 Answer: {}", task2.count());
}
