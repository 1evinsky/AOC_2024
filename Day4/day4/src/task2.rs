pub struct Task2 {
    patterns: Vec<String>,
    lines: Vec<String>
}

impl Task2
{
    pub fn new(patterns: Vec<String>, lines: Vec<String>) -> Task2
    {
        Task2
        {
            patterns,
            lines
        }
    }

    pub fn count(&self) -> usize
    {
        let mut count_total: usize = 0;

        for (line_index, line) in self.lines.iter().enumerate() 
        {
            if line_index == 0
            {
                continue;
            }

            if line_index == self.lines.len() - 1
            {
                continue;
            }

            for (char_index, ch) in line.chars().enumerate()
            {
                if char_index == 0
                {
                    continue;
                }

                if char_index == line.len() - 1
                {
                    continue;
                }

                if ch != 'A'
                {
                    continue;
                }

                let left_top: char = self.lines[line_index - 1].chars().nth(char_index - 1).unwrap();
                let right_top: char = self.lines[line_index - 1].chars().nth(char_index + 1).unwrap();
                let left_bottom: char = self.lines[line_index + 1].chars().nth(char_index - 1).unwrap();
                let right_bottom: char = self.lines[line_index + 1].chars().nth(char_index + 1).unwrap();

                let pattern1: Vec<char> = vec![left_top, ch, right_bottom];
                let pattern1: String = pattern1.iter().collect();

                let pattern2: Vec<char> = vec![right_top, ch, left_bottom];
                let pattern2: String = pattern2.iter().collect();

                if self.patterns.contains(&pattern1) && self.patterns.contains(&pattern2)
                {
                    count_total += 1;
                }
            }
        }

        count_total
    }
}