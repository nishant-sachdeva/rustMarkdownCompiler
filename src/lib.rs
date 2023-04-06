use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::io::Write;

pub struct MarkdownCompiler {
    pub filename: String,
}

impl MarkdownCompiler {
    pub fn new(filename: &str) -> MarkdownCompiler {
        MarkdownCompiler {
            filename: String::from(filename),
        }
    }

    fn parse_markdown(&self, reader: &mut BufReader<File>) -> Vec<String> {
        let mut tokens = Vec::new();
        for line in reader.lines() {
            let mut output_line = String::new();
            let line_contents = line.unwrap();
 
            if line_contents.starts_with("#") {               
                output_line.push_str("<h1>");
                output_line.push_str(&line_contents[2..]);
                output_line.push_str("</h1>\n");

            } else {
                output_line.push_str("<p>");
                output_line.push_str(&line_contents);
                output_line.push_str("</p>\n");
            }

            if output_line != "<p></p>\n" {
                tokens.push(output_line);
            }
        }
        return tokens;
    }

    fn create_output(&self, tokens: Vec<String>) {
        let mut output_filename = String::from(&self.filename[..self.filename.len() - 3]);
        output_filename.push_str(".html");

        let mut output_file = File::create(
            Path::new(&output_filename)
        )
        .expect("Could not create output file");

        for t in tokens.iter() {
            output_file.write_all(
                t.as_bytes()
            )
            .expect("Could not write to output file");
        }

        println!("[ INFO ] Parsing complete!. Output available at {}", output_filename);
    }

    fn check_input(&self) -> BufReader<File> {
        let file = File::open(
            Path::new(&self.filename)
        )
        .expect("File not found");
    
        let reader = BufReader::new(file);

        return reader;
    }

    pub fn compile(&mut self) {
        let mut reader = self.check_input();

        let tokens = self.parse_markdown(&mut reader);

        self.create_output(tokens);
    }
}
