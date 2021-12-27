use crate::lexer::*;
use crate::token::*;

#[derive(Debug)]
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
                    self.get_next();
                }
            }
            _ => (),
        }
        return local_token;
    }
    pub fn peek_next(&mut self) -> TOKEN {
        let mut local_token = TOKEN::EOF;
        match self.slice[self.index..].chars().next() {
            Some(c) => {
                local_token = get_token(c);
                if local_token == TOKEN::Empty {
                    self.index += seek_past_whitespace(self.get_slice());
                    return self.peek_next();
                }
            }
            _ => (),
        }
        return local_token;
    }
    pub fn skip_empty(&mut self) -> () {
        self.index += seek_past_whitespace(self.get_slice());
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
    pub fn reset(&mut self) -> () {
        self.index = self.prev;
        self.skip_empty();
    }
}
