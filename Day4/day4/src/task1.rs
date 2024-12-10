pub struct Task1
{
    patterns: Vec<String>,
    lines: Vec<String>
}

impl Task1
{
    pub fn new(patterns: Vec<String>, lines: Vec<String>) -> Task1
    {
        Task1
        {
            patterns,
            lines
        }
    }

    fn count_substrings(&self, line: &String) -> usize 
    {
        let mut total_count = 0;

        for pattern in &self.patterns 
        {
            total_count += line.matches(pattern).count();
        }

        total_count
    }

    pub fn count(&self) -> usize
    {
        let mut count_total: usize = 0;
        let line_len: usize = self.lines[0].len();
        let mut column: Vec<String> = vec![String::new(); line_len]; 
        let mut diagonal_1: Vec<String> = vec![String::new(); self.lines.len() + line_len - 1]; 
        let mut diagonal_2: Vec<String> = vec![String::new(); self.lines.len() + line_len - 1]; 

        for (line_index, line) in self.lines.iter().enumerate() 
        {
            let count: usize = self.count_substrings(line);
            count_total += count;

            for(char_index, ch) in line.chars().enumerate()
            {
                column[char_index].push(ch);
                diagonal_1[line_index + char_index].push(ch);
                diagonal_2[(char_index as isize - line_index as isize + line_len as isize - 1) as usize].push(ch);
            }
        }

        for (_index, line) in column.iter().enumerate()
        {
            let count: usize = self.count_substrings(line);
            count_total += count;
        }

        for (_index, line) in diagonal_1.iter().enumerate()
        {
            let count: usize = self.count_substrings(line);
            count_total += count;
        }

        for (_index, line) in diagonal_2.iter().enumerate()
        {
            let count: usize = self.count_substrings(line);
            count_total += count;
        }

        count_total
    }
}