use crate::lexer::*;
use crate::token::*;

pub struct Tracker<'a> {
    slice: &'a str,
    prev: usize,
    index: usize,
}

impl<'a> Tracker<'a> {
    pub fn new(slice: &'a str) -> Self {
        Tracker {
            slice,
            prev: 0,
            index: 0,
        }
    }
    pub fn adv(&mut self, inc: usize) -> () {
        self.prev = self.index;
        self.index += inc;
    }
    pub fn get_next(&mut self) -> TOKEN {
        let mut local_token = TOKEN::EOF;
        match self.slice[self.index..].chars().next() {
            Some(c) => {
                self.index += 1;
                local_token = get_token(c);
                if local_token == TOKEN::Empty {
                    self.index += seek_past_whitespace(self.get_slice());
                    println!("{}", self.get_slice());
                }
            }
            _ => (),
        }
        return local_token;
    }
    pub fn get_slice(&self) -> &'a str {
        return &self.slice[self.index..];
    }
    pub fn prev(&self) -> usize {
        return self.prev;
    }
    pub fn current(&self) -> usize {
        return self.index;
    }
}
