// Table formatting utilities

#[allow(dead_code)]
pub struct TableBuilder {
    rows: Vec<Vec<String>>,
    headers: Vec<String>,
    widths: Vec<usize>,
}

impl TableBuilder {
    #[allow(dead_code)]
    pub fn new(headers: Vec<&str>) -> Self {
        let headers: Vec<String> = headers.iter().map(|&h| h.to_string()).collect();
        let widths = headers.iter().map(|h| h.len()).collect();

        Self {
            rows: Vec::new(),
            headers,
            widths,
        }
    }

    #[allow(dead_code)]
    pub fn row(mut self, values: Vec<&str>) -> Self {
        let row: Vec<String> = values.iter().map(|&v| v.to_string()).collect();

        // Update column widths
        for (i, val) in row.iter().enumerate() {
            if i < self.widths.len() && val.len() > self.widths[i] {
                self.widths[i] = val.len();
            }
        }

        self.rows.push(row);
        self
    }

    #[allow(dead_code)]
    pub fn build(self) -> String {
        let mut output = String::new();

        // Top border
        let total_width: usize = self.widths.iter().sum::<usize>() + (self.widths.len() * 3) + 1;
        output.push_str(&format!("╔{}╗\n", "═".repeat(total_width - 2)));

        // Headers
        output.push_str("║ ");
        for (i, header) in self.headers.iter().enumerate() {
            let width = self.widths.get(i).copied().unwrap_or(header.len());
            output.push_str(&format!("{:width$} │ ", header, width = width));
        }
        output.pop(); // Remove last " │ "
        output.pop();
        output.pop();
        output.push_str(" ║\n");

        // Separator
        output.push_str(&format!("╠{}╣\n", "═".repeat(total_width - 2)));

        // Rows
        for row in self.rows {
            output.push_str("║ ");
            for (i, val) in row.iter().enumerate() {
                let width = self.widths.get(i).copied().unwrap_or(val.len());
                output.push_str(&format!("{:width$} │ ", val, width = width));
            }
            output.pop(); // Remove last " │ "
            output.pop();
            output.pop();
            output.push_str(" ║\n");
        }

        // Bottom border
        output.push_str(&format!("╚{}╝", "═".repeat(total_width - 2)));

        output
    }
}
