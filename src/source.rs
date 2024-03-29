use alloc::string::String;

use crate::error::{CompilerError, Result};

#[derive(Debug)]
pub struct Source {
    text: String,
}

impl Source {
    pub fn new(text: String) -> Source {
        Source { text }
    }

    pub fn line_index(&self, index: usize) -> usize {
        if index == 0 {
            return 0;
        }

        //(index + self.text[..index].lines().count() - 1) % self.text.lines().count()
        self.text[..index].lines().count() - 1
    }

    pub fn line(&self, index: usize) -> Result<&str> {
        self.text
            .lines()
            .nth(index)
            .ok_or(CompilerError::ReadSource)
    }

    pub fn line_start(&self, index: usize) -> usize {
        self.text
            .lines()
            .take(index)
            .map(|line| line.len() + 1)
            .sum()
    }
}
