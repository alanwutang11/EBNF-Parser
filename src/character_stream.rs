use std::fs::File;
use std::io;
use std::convert::TryFrom;
use std::io::prelude::*;

pub struct CharStream {
	inp_str: String,
	cur_char_idx: u32,
	inp_str_len: usize,
	cur_line_no: u32,
}

impl CharStream {

	pub fn new(f: &str) -> CharStream {
		CharStream {
			inp_str: f.to_string(),
			cur_char_idx: 0,
			inp_str_len: f.to_string().len(),
			cur_line_no: 0,
		}
	}
	
	// Returns true if more characters are available, false otherwise.
	pub fn more_available(&self) -> bool {
		let next_idx = self.cur_char_idx + 1;
		if self.get_idx() == (self.get_len() - 1).try_into().unwrap() {
			return false;
		}
		return self.inp_str.is_char_boundary(next_idx.try_into().unwrap());
		
	}

	// Returns the next character without consuming it.
	// Returns None if no more characters are available. 
	pub fn peek_next_char(&self) -> Option<char> {
		if self.more_available() {
			let next_idx = self.cur_char_idx + 1;
			return Some(self.inp_str.chars().nth(next_idx.try_into().unwrap()).unwrap());
		}
		else {
			return None;
		}
		
	}

	// Returns the kth character ahead in the stream without consuming it.
	// peek_ahead_char(0) returns the same character as peek_next_char().
	// Returns None if no more characters are available at the position.
	// The input k cannot be negative.
	pub fn peek_ahead_char(&self, k: i32) -> Option<char> {
		//k should be greater than the current character index. 
		
		let k = k + 1;
		if self.more_available() && self.cur_char_idx <= k.try_into().unwrap() {
			return Some(self.inp_str.chars().nth(k.try_into().unwrap()).unwrap());
		}
		else {
			return None;
		}
	}

	// Returns the next character and consumes it.
	// Returns None if no more characters are available.
	pub fn get_next_char(&mut self) -> Option<char> {

		if self.more_available() {
			if self.get_cur_char().unwrap() == '\n' {
				self.cur_line_no = self.cur_line_no + 1;
			}
			self.cur_char_idx = self.cur_char_idx + 1;
			
			return Some(self.inp_str.chars().nth(self.cur_char_idx.try_into().unwrap()).unwrap());
		}
		else {
			return None;
		}
	}

	//I added the below functions

	//gets the current character that the index points to
	pub fn get_cur_char(&mut self) -> Option<char> {
		if self.more_available() {
			return Some(self.inp_str.chars().nth(self.cur_char_idx.try_into().unwrap()).unwrap());
		}
		else {
			return None;
		}
	} 

	pub fn get_idx(&self) -> u32 {
		return self.cur_char_idx;
	}

	pub fn get_line_no(&self) -> u32 {
		return self.cur_line_no;
	}

	pub fn get_len(&self) -> usize {
		return self.inp_str_len;
	}

}



