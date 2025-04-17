use std::io::Error;
use std::fs::read_to_string;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty() 
    }

    pub fn load(filepath: &str) -> Result<Self, Error> {
        let contents = read_to_string(filepath)?;
        let mut lines = Vec::new();

        for line in contents.lines() {
            lines.push(String::from(line));
        }

        Ok(Self { lines })
    }
}

// impl Default for Buffer {
//     fn default() -> Buffer {
//         Buffer {
//             lines: vec![String::from("Hello world")],
//         }
//     }
// }

