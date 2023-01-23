use crate::token::*;
use crate::character_stream::*;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

//open the files from the command line, 
//tokenize the text of the files into:

//1) operators 
//2) intConstants, 
//3) floatConstants, 
//4) keywords, 
//5) and identifiers.

//handle incorrect numbers for the scanner
pub struct Scanner {

    token_stream: Vec<Token>,
    num_tokens: u32,
    char_stream: CharStream,
    lookup: HashMap<String, i32>,

    cur_token_num: i32, //token pointer


}

impl Scanner {

    pub fn new(f: &str) -> Scanner {
		
        let mut token_vec: Vec<Token> = Vec::new();

        //reading from file input 
	    //https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
	    let file = File::open(f).expect("open file fail");

	    let mut data = String::new();
	    let mut br = io::BufReader::new(file);
	    br.read_to_string(&mut data).expect("read string fail");

	    let mut new_char_stream = CharStream::new(&data);

        //0 -> keyword
        //1 -> operator
        let new_lookup = HashMap::from([
            ("unsigned".to_string(), 0),
            ("char".to_string(), 0),
            ("short".to_string(), 0),
            ("int".to_string(), 0),
            ("long".to_string(), 0),
            ("float".to_string(), 0),
            ("double".to_string(), 0),
            ("while".to_string(), 0),
            ("if".to_string(), 0),
            ("return".to_string(), 0),
            ("void".to_string(), 0),
            ("main".to_string(), 0),

            ("(".to_string(), 1),
            (",".to_string(), 1),
            (")".to_string(), 1),
            ("{".to_string(), 1),
            ("}".to_string(), 1),
            ("=".to_string(), 1),
            ("==".to_string(), 1),
            ("<".to_string(), 1),
            (">".to_string(), 1),
            ("<=".to_string(), 1),
            (">=".to_string(), 1),
            ("!=".to_string(), 1),
            ("+".to_string(), 1),
            ("-".to_string(), 1),
            ("*".to_string(), 1),
            ("/".to_string(), 1),
            (";".to_string(), 1)
        ]);


        //check if the string is empty and throw error if it is 
        assert!(new_char_stream.more_available()); 
        
        Scanner {

            token_stream: token_vec,
            num_tokens: 0,
            char_stream: new_char_stream,
            lookup: new_lookup,
            cur_token_num: -1, 
        }

	}
 
    pub fn check_more_available(&self) -> bool {
        return self.char_stream.more_available();
    }
    

    pub fn get_class(&self, s: char) -> u32 { 
        if s.is_alphabetic() || s == '_' {
            return 0;
        } 
        else if s.is_numeric() {
            return 1;
        }
        else if s == '-' {
            return 3;
        }
        else {
            return 2;
        }
    }
    

    pub fn get_non_blank(&mut self) -> char {


        while self.char_stream.peek_next_char().unwrap() == ' ' ||
        self.char_stream.peek_next_char().unwrap() == '\n' ||
        self.char_stream.peek_next_char().unwrap() == '\t' {
            self.char_stream.get_next_char();
            if (self.char_stream.get_idx() == (self.char_stream.get_len() - 1).try_into().unwrap()) {
                return ' ';
            }
        }

        return self.char_stream.get_cur_char().unwrap(); 
    }

    pub fn add_token(&mut self, lex_str: &String, mut tt_int: i32) -> () {
        let tt;

        if self.lookup.contains_key(lex_str) {
            tt_int = self.lookup.get(lex_str).unwrap().clone();
        }


        if tt_int == 0 {
            tt = TokenType::KEYWORD;
        }
        else if tt_int == 1 {
            tt = TokenType::OPERATOR;
        }
        else if tt_int == 2 {
            tt = TokenType::INTCONSTANT;
        }
        else if tt_int == 3 {
            tt = TokenType::FLOATCONSTANT;
        }
        else if tt_int == 4 {
            tt = TokenType::VARIABLE;
        }
        else {
            tt = TokenType::INVALID; 
        }
        let token = Token::new(lex_str.clone(), 
                tt,
                self.char_stream.get_line_no() as i32, 
                self.char_stream.get_idx() as i32);

        println!("________________");
        println!("text: {}", token.get_text());
        println!("token type: {}", token.get_type().as_str());
        println!("________________");

        self.token_stream.push(token);
        self.num_tokens = self.num_tokens + 1;

        
    }

    //there should be one lex function call for every lexeme 
    pub fn lex(&mut self) -> () {
        let mut lex_str = String::new(); //string to store the lex, should be a Token object

        //edge case stuff 
        if self.char_stream.get_idx() == 0 {
            lex_str.push(self.char_stream.get_cur_char().unwrap());
        }
        else {
            lex_str.push(self.char_stream.get_next_char().unwrap());
        }

        let str_it = 0; //string iterator to index 
        let first_char_class = self.get_class(lex_str.chars().nth(str_it).unwrap());

        
        match first_char_class {
            0 => {
                //add so it checks for underscore as well. 
                while self.get_class(self.char_stream.peek_next_char().unwrap()) == 0 || 
                self.get_class(self.char_stream.peek_next_char().unwrap()) == 1 {
                    lex_str.push(self.char_stream.get_next_char().unwrap());
                } 
                
                self.add_token(&lex_str, 4);
            },
            1 | 3 => {
                let mut float_flag = 0;
                
                while self.get_class(self.char_stream.peek_next_char().unwrap()) == 1 {
                    lex_str.push(self.char_stream.get_next_char().unwrap()); 
                }

                if self.char_stream.peek_next_char().unwrap() == '.' {
                    float_flag = 1;
                    lex_str.push(self.char_stream.get_next_char().unwrap());
                }
        
                if float_flag == 1 {
                    while self.get_class(self.char_stream.peek_next_char().unwrap()) == 1 {
                        lex_str.push(self.char_stream.get_next_char().unwrap()); 
                    }
                    self.add_token(&lex_str, 3);
                }
                else {
                    self.add_token(&lex_str, 2);
                }
            },
            2 => {
                if self.char_stream.peek_next_char().unwrap() == '=' {
                    lex_str.push(self.char_stream.get_next_char().unwrap());
                }
                self.add_token(&lex_str,-1);
                
            },
            _ => {},
        }

    }

    pub fn get_num_tokens(&self) -> u32 {
        return self.num_tokens;
    } 
    
    pub fn get_cur_num_token(&self) -> i32 {
        return self.cur_token_num;
    }  

    pub fn get_next_token(&mut self) -> Option<Token> {
        self.cur_token_num = self.cur_token_num + 1;
        if self.num_tokens <= self.cur_token_num.try_into().unwrap() {
            return None;
        }
        else {
            return Some(self.token_stream.remove(0));
        }
    }

    pub fn peek_ahead_token(&mut self, i: usize) -> Option<String> {
        if self.num_tokens <= (self.cur_token_num + 1).try_into().unwrap() {
            return None;
        }
        else {
            return Some(self.token_stream.get(i).unwrap().get_text().to_string());
        }
    }

}