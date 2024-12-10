use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct FileReader {
    file_path: String
}

impl FileReader {
    pub fn new(file_path: &str) -> FileReader {
        FileReader {
            file_path: file_path.to_string()
        }
    }

    pub fn read_by_lines(&self) -> io::Result<Vec<String>> {
        let file: File = File::open(&self.file_path)?;
        let reader: BufReader<File> = BufReader::new(file);

        reader.lines().collect()
    }
}