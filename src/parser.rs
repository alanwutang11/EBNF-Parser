
use crate::token::*;
use crate::scanner::*;

pub struct Parser {
    scanner: Scanner,
    cur_token: Token,
    parsed_token_stream: Vec<Token>,
    num_parsed: i32,
    cur_token_num: i32,
}

impl Parser {

    pub fn new(mut s: Scanner) -> Parser {

        assert!(s.get_num_tokens() > 0); 
        let first_token = s.get_next_token().unwrap();
        let mut parsed_token_vec: Vec<Token> = Vec::new();
        
        Parser {
            scanner: s,
            cur_token: first_token,
            parsed_token_stream: parsed_token_vec,
            num_parsed: 0,
            cur_token_num: -1, 
        }
    }

    pub fn inc_token_ptr(&mut self) -> () {

        if self.scanner.get_cur_num_token() == (self.scanner.get_num_tokens() - 1).try_into().unwrap() {
            return;
        }
        else {
            self.cur_token = self.scanner.get_next_token().unwrap();
        }
    }

    pub fn add_parsed_token(&mut self) -> () {

        let tt;

        if self.cur_token.get_type().as_str().eq("None") {
            tt = TokenType::NONE;
        }
        else if self.cur_token.get_type().as_str().eq("IntConstant") {
            tt = TokenType::INTCONSTANT;
        }
        else if self.cur_token.get_type().as_str().eq("FloatConstant") {
            tt = TokenType::FLOATCONSTANT;
        }
        else if self.cur_token.get_type().as_str().eq("Keyword") {
            tt = TokenType::KEYWORD;
        }
        else if self.cur_token.get_type().as_str().eq("Variable") {
            tt = TokenType::VARIABLE;
        }
        else if self.cur_token.get_type().as_str().eq("Function") {
            tt = TokenType::FUNCTION;
        }
        else if self.cur_token.get_type().as_str().eq("Operator") {
            tt = TokenType::OPERATOR;
        }
        else {
            tt = TokenType::NONE;
        }

        let token = Token::new(self.cur_token.get_text().to_string(), 
                tt,
                self.cur_token.get_line_number(), 
                self.cur_token.get_char_pos());
        self.parsed_token_stream.push(token);
        self.num_parsed = self.num_parsed + 1;
    }

    pub fn get_num_parsed(&self) -> i32 {
        return self.num_parsed;
    }

    pub fn expect_str(&mut self, s: String) -> bool {
        if self.cur_token.get_text().eq(&s) {
            // println!("cur token: {}", self.cur_token.get_text());
            //clone token and add it to the token vector 
            self.add_parsed_token();
            self.inc_token_ptr();
            // println!("new token: {}", self.cur_token.get_text());
            return true;
        }
        else {
            return false;
        }
    }

    pub fn expect_tok_type(&mut self, tt: TokenType) -> bool {
        if tt.as_str().eq(self.cur_token.get_type().as_str()) {
            // println!("cur token: {}", self.cur_token.get_text());
            self.add_parsed_token();
            self.inc_token_ptr();
            // println!("new token: {}", self.cur_token.get_text());
            return true;
        }
        else {
            return false;
        }
    }

    pub fn program(&mut self) -> () {
        while self.check_decl() {
            self.decl();
        }
        let main_decl = self.main_decl();
        if !main_decl {
            panic!("invalid/missing main declaration");
        }
        while self.check_decl() {
            self.func_def();
        }
    }

    pub fn check_decl(&mut self) -> bool {
        //for a while loop call, peek next token to see if it equals the first decl. if it does, call the function and confirm that the 
        //rest of decl works. return true if the first token is indeed the first token of a valid decl without comsuming that token
        if self.cur_token.get_text().eq("unsigned") {
            let peek_ahead_option = self.scanner.peek_ahead_token(1);
            if let Some(ref i) = peek_ahead_option {
                if i.eq(&"char".to_string()) || i.eq(&"short".to_string()) ||
                i.eq(&"int".to_string()) || i.eq(&"long".to_string()) {
                    return true;
                }
                else {
                    return false;
                }
            }
        }
        
        if self.cur_token.get_text().eq("char") || self.cur_token.get_text().eq("short") ||
        self.cur_token.get_text().eq("int") || self.cur_token.get_text().eq("long") || 
        self.cur_token.get_text().eq("float") || self.cur_token.get_text().eq("double") {
            return true;
        }
        else {
            return false;
        }
        
    }
    
    pub fn decl(&mut self) -> bool {

        let decl_type = self.decl_type();
        if !decl_type {
            panic!("expected data type on line: {}", self.cur_token.get_line_number());
        }

        if self.cur_token.get_text().eq("=") || self.cur_token.get_text().eq(";") {
            if !self.var_decl(){
                panic!("expected variable declaration on line: {}", self.cur_token.get_line_number());
            }
            else {
                return true;
            }
        }
        else if self.cur_token.get_text().eq("("){
            if !self.func_decl(){
                panic!("expected function declaration on line: {}", self.cur_token.get_line_number());
            }
            else {
                return true;
            }

        }
        else {
            panic!("missing either \"=\", \";\", or \"(\", : {}", self.cur_token.get_line_number());
        }
    }
    
    pub fn main_decl(&mut self) -> bool {
        let void_keyword = self.expect_str("void".to_string());
        if !void_keyword {
            panic!("main declaration is missing keyword \"void\" on line: {}", self.cur_token.get_line_number());
        }
        let main_keyword = self.expect_str("main".to_string());
        if !main_keyword {
            panic!("main declaration is missing keyword \"main\" on line: {}", self.cur_token.get_line_number());
        }
        let open_paren = self.expect_str("(".to_string());
        if !open_paren {
            panic!("main declaration is missing operator \"(\" on line: {}", self.cur_token.get_line_number());
        }
        let close_paren = self.expect_str(")".to_string());
        if !close_paren {
            panic!("main declaration is missing operator \")\" on line: {}", self.cur_token.get_line_number());
        }
        let block = self.block();
        if !block {
            panic!("block expected on line: {}", self.cur_token.get_line_number());
        }

        return true;
    }
    
    pub fn func_def(&mut self) -> bool {
        let decl_type = self.decl_type();
        
        if !decl_type {
            panic!("type expected for line: {}", self.cur_token.get_line_number());
        }
        let param_block = self.param_block();

        if !param_block {
            panic!("param block expected for a function definition on line: {}", self.cur_token.get_line_number());
        }
        let block = self.block();
        if !block {
            panic!("block expected for a function definition on line: {}", self.cur_token.get_line_number());
        }

        return true;
    }

    pub fn decl_type(&mut self) -> bool {
        let dt = self.data_type();
        if !dt {
            return false;
        }
        
        let mut tt = TokenType::VARIABLE;
        let peek_ahead_option = self.scanner.peek_ahead_token(0);
        if let Some(ref _i) = peek_ahead_option {
            if peek_ahead_option.unwrap().eq("(") {
                self.cur_token.set_token_type(TokenType::FUNCTION);
                tt = TokenType::FUNCTION;
            }
        }
        let ident = self.expect_tok_type(tt);
        
        
        if !ident {
            panic!("identifier expected on line: {}", self.cur_token.get_line_number());
        }
        else {
            return true;
        }
    }
    
    pub fn var_decl(&mut self) -> bool {
        
        if self.expect_str("=".to_string()) {
            if !self.constant() {
                panic!("expected constant after \"=\" on line: {}", self.cur_token.get_line_number());
            }
        }
        let semi_cln = self.expect_str(";".to_string());
        
        //if function declaration is also false, this should be an error 
        if !semi_cln {
            panic!("expected \";\" on line: {}", self.cur_token.get_line_number());
        }
        else {
            return true;
        } 
    }
    
    pub fn func_decl(&mut self) -> bool {
        let param_blk = self.param_block();

        if !param_blk {
            return false;
        }
        let semi_cln = self.expect_str(";".to_string());
        if !semi_cln {
            panic!("expected \";\" on line: {}", self.cur_token.get_line_number());
        }
        
        return true;
    }
    
    pub fn block(&mut self) -> bool {
        let open_bracket = self.expect_str("{".to_string());
        if !open_bracket {
            panic!("missing operator \"{{\" on line: {}", self.cur_token.get_line_number());
        }

        while self.check_decl() {
            if !self.decl() {
                panic!("missing/invalid declaration on line: {}", self.cur_token.get_line_number());
            }
        }
        
        while self.cur_token.get_text().eq("while") || self.cur_token.get_text().eq("if") ||
        self.cur_token.get_text().eq("return") || self.cur_token.get_text().eq("(") ||  
        self.cur_token.get_type().as_str().eq("Variable") {
          if !self.stmnt() {
            panic!("missing/invalid statement on line: {}", self.cur_token.get_line_number());
          } 
        }

        while self.check_decl() {
            if !self.func_def() {
                panic!("missing/invalid function definition on line: {}", self.cur_token.get_line_number());
            }
        }
  
        let close_bracket = self.expect_str("}".to_string());
        if !close_bracket {
            panic!("missing operator \"}}\" on line: {}", self.cur_token.get_line_number());
        }

        return open_bracket && close_bracket;
    }

    pub fn param_block(&mut self) -> bool {
        let open_paren = self.expect_str("(".to_string());
        if !open_paren {
            panic!(" \"(\" expected for param block on line: {}", self.cur_token.get_line_number());
        }

        if self.param() {
            while self.expect_str(",".to_string()) && self.param() {}
        }

        let close_paren = self.expect_str(")".to_string());
        if !close_paren {
            panic!("missing operator \")\" on line: {}", self.cur_token.get_line_number());
        }
        else {
            return true;
        }
    }

    pub fn data_type(&mut self) -> bool {
        if self.int_type() || self.float_type() {
            return true;
        }
        else {
            return false;
        }
    }
    
    pub fn constant(&mut self) -> bool {

        let float_tt = TokenType::FLOATCONSTANT;
        let int_tt = TokenType::FLOATCONSTANT;

        if self.expect_tok_type(float_tt) || self.expect_tok_type(int_tt) {
            return true;
        }
        else {
            return false;
        }

    }
    
    pub fn stmnt(&mut self) -> bool {

        let peek_ahead_option = self.scanner.peek_ahead_token(0);
        if let Some(ref _i) = peek_ahead_option {
            if self.cur_token.get_type().as_str().eq("Variable") && peek_ahead_option.unwrap().eq("=") {
                let assign = self.assign();
                if !assign {
                    panic!("expected assignment on line: {}", self.cur_token.get_line_number());
                }
                else {
                    return true;
                }
            }
        }
        

        if self.cur_token.get_text().eq("while") {
            let while_loop = self.while_loop();
            if while_loop {
                return true;
            }
            else {
                return false;
            }
        }
        else if self.cur_token.get_text().eq("if") {
            let if_stmnt = self.if_stmnt();
            if if_stmnt {
                return true;
            }
            else {
                return false;
            }
        }
        else if self.cur_token.get_text().eq("return") {
            let ret_stmnt = self.ret_stmnt();
            if ret_stmnt {
                return true;
            }
            else {
                return false;
            }
        }
        else {
            let expr = self.expr();
            if expr {
                if !self.expect_str(";".to_string()) {
                    panic!("expected \";\" on line: {}", self.cur_token.get_line_number());
                }
                else {
                    return true;
                }
            }
            return false;
        }
    }
    
    pub fn param(&mut self) -> bool {
        let ret_dt = self.data_type();
        if !ret_dt {
            return false;
        }
        
        let tt = TokenType::VARIABLE;
        let ret_ident = self.expect_tok_type(tt);
        
        if !ret_ident {
            panic!("expected identifier on line: {}", self.cur_token.get_line_number());
        }
        else {
            return true;
        }
        
    }

    pub fn int_type(&mut self) -> bool {
        let mut seen_unsigned = false;
        if self.expect_str("unsigned".to_string()) {
            seen_unsigned = true;
        }
        if self.expect_str("char".to_string()) || self.expect_str("short".to_string()) ||
            self.expect_str("int".to_string()) || self.expect_str("long".to_string()) {
                return true;
        }
        else {
            if seen_unsigned {
                panic!("expected data type after \"unsigned\" keyword on line: {}", self.cur_token.get_line_number());
            }
                return false;
        }
    }
    
    pub fn float_type(&mut self) -> bool {
        if self.expect_str("float".to_string()) || self.expect_str("double".to_string()) {
            return true;
        }
        else {
            return false;
        }
    }
    
    pub fn assign(&mut self) -> bool {
        
        let tt = TokenType::VARIABLE;
        if !self.expect_tok_type(tt) {
            panic!("expected variable on line: {}", self.cur_token.get_line_number());
        }
        
    
        let equal = self.expect_str("=".to_string());
        if !equal {
            panic!("missing \"=\" after assignment on line: {}", self.cur_token.get_line_number());
        }
        
        //check the current and next. If both equal, then accept, increment pointer TWICE and try again.

        if !self.scanner.peek_ahead_token(0).is_none() {
            let mut next_str = self.scanner.peek_ahead_token(0).unwrap();
            while TokenType::VARIABLE.as_str().eq(self.cur_token.get_type().as_str()) && next_str.eq("=") {
                self.inc_token_ptr();
                self.inc_token_ptr();
                if self.scanner.peek_ahead_token(0).is_none() {
                    break;
                }
                next_str = self.scanner.peek_ahead_token(0).unwrap();
            }
        }

        let expr = self.expr();
        if !expr {
            panic!("expression expected on line: {}", self.cur_token.get_line_number());
        }
        let semi_cln = self.expect_str(";".to_string());
        if !semi_cln {
            panic!("expected \";\" on line: {}", self.cur_token.get_line_number());
        }
        return true;
    }
    
    pub fn while_loop(&mut self) -> bool {

        let while_keyword = self.expect_str("while".to_string());
        if !while_keyword {
            panic!("while loop is missing keyword \"while\" on line: {}", self.cur_token.get_line_number());
        }
        let open_paren = self.expect_str("(".to_string());
        if !open_paren {
            panic!("while loop is missing operator \"(\" on line: {}", self.cur_token.get_line_number());
        }
        let expr = self.expr();
        if !expr {
            panic!("while loop is missing an expression on line: {}", self.cur_token.get_line_number());
        }
        let close_paren = self.expect_str(")".to_string());
        if !close_paren {
            panic!("while loop is missing operator \")\" on line: {}", self.cur_token.get_line_number());

        }
        let block = self.block();
        if !block {
            panic!("block expected for if statement on line: {}", self.cur_token.get_line_number());
        }
        return true;  
    }

    pub fn if_stmnt(&mut self) -> bool {
        let if_keyword = self.expect_str("if".to_string());
        if !if_keyword {
            panic!("if statement is missing keyword \"if\" on line {}", self.cur_token.get_line_number());
        }
        let open_paren = self.expect_str("(".to_string());
        if !open_paren {
            panic!("if statement is missing operator \"(\" on line: {}", self.cur_token.get_line_number());
        }
        let expr = self.expr();
        if !expr {
            panic!("expression expected after \"return\" on line: {}", self.cur_token.get_line_number());
        }
        let close_paren = self.expect_str(")".to_string());
        if !close_paren {
            panic!("if statement is missing operator \")\" on line: {}", self.cur_token.get_line_number());
        }
        let block = self.block();
        if !block {
            panic!("block expected for if statement on line: {}", self.cur_token.get_line_number());
        }
        return true;
    }
    
    pub fn ret_stmnt(&mut self) -> bool {
        self.expect_str("return".to_string());
        let expr = self.expr();
        if !expr {
            panic!("expression expected after \"return\" on line: {}", self.cur_token.get_line_number());
        }
        let semi = self.expect_str(";".to_string());
        if !semi {
            panic!("expected \";\" on line: {}", self.cur_token.get_line_number());
        }
        return true;
    }
    
    pub fn expr(&mut self) -> bool {
        let smpl_expr = self.smpl_expr();
       
        if self.rel_op() {
            if self.cur_token.get_type().as_str().eq("IntConstant") || self.cur_token.get_type().as_str().eq("FloatConstant") || 
            self.cur_token.get_text().eq("(") || self.cur_token.get_type().as_str().eq("Variable") {
                let inner_smpl_expr = self.smpl_expr();
                if !inner_smpl_expr {
                    panic!("valid simple expression expected after relation operator on line: {}", self.cur_token.get_line_number());
                }
            }
        }

        return smpl_expr;
    }
    
    pub fn smpl_expr(&mut self) -> bool {
        let term = self.term();

        while self.cur_token.get_text().eq("+") || self.cur_token.get_text().eq("-") {
            if !self.add_op() {
                panic!("missing \"+\" or \"-\" on line: {}", self.cur_token.get_line_number());
            }
            let peek_ahead_option = self.scanner.peek_ahead_token(0);
            if let Some(_i) = peek_ahead_option {
                if !self.term() {
                    panic!("invalid/missing term on line: {}", self.cur_token.get_line_number());
                }
                else {
                    break;
                }
            }
            else {
                break;
            }
        }
        return term;
    }

    pub fn term(&mut self) -> bool {
        let factor = self.factor();
        
        while self.cur_token.get_text().eq("*") || self.cur_token.get_text().eq("/") {
            if !self.mult_op() {
                panic!("missing \"*\" or \"/\" on line: {}", self.cur_token.get_line_number());
            }
            let peek_ahead_option = self.scanner.peek_ahead_token(0);
            if let Some(_i) = peek_ahead_option {
                if !self.factor() {
                    panic!("invalid/missing factor on line: {}", self.cur_token.get_line_number());
                }
                else {
                    break;
                }
            }
            else {
                break;
            }
        }
        return factor;    
    }

    pub fn factor(&mut self) -> bool {

        let mut var_func_type = TokenType::VARIABLE;
        let float_type = TokenType::FLOATCONSTANT;
        let int_type = TokenType::INTCONSTANT;

        if self.expect_tok_type(float_type) || self.expect_tok_type(int_type) {
            return true;
        }
        else if self.cur_token.get_type().as_str().eq("Variable") {
            let peek_ahead_option = self.scanner.peek_ahead_token(0);
            if let Some(ref _i) = peek_ahead_option {
                if peek_ahead_option.unwrap().eq("(") {
                    self.cur_token.set_token_type(TokenType::FUNCTION);
                    var_func_type = TokenType::FUNCTION;
                }
            }
            if !self.expect_tok_type(var_func_type) {
                panic!("missing identifier on line: {}", self.cur_token.get_line_number());
            }
            let open_paren_two = self.expect_str("(".to_string());
            if open_paren_two {
                if !self.expr() {
                    return false;
                }
                //double check if the while loop needs to be in the return.
                while self.expect_str(",".to_string()) && self.expr() {} 
                let close_paren_two = self.expect_str(")".to_string());
                if !close_paren_two {
                    panic!("missing operator \")\" on line: {}", self.cur_token.get_line_number());
                }
            }
            
            return true;
        }
        else if self.expect_str("(".to_string()) {
            let expr = self.expr();
            if !expr {
                return false;
            }
            let close_paren = self.expect_str(")".to_string());
            if !close_paren {
                panic!("missing operator \")\" on line: {}", self.cur_token.get_line_number());
            }
            else {
                return true;
            }
        }
        else {
            return false;
        }
    }
    
    pub fn rel_op(&mut self) -> bool {
        if self.expect_str("==".to_string()) || self.expect_str(">".to_string()) ||
        self.expect_str("<".to_string()) || self.expect_str("<=".to_string()) ||
        self.expect_str(">=".to_string()) || self.expect_str("!=".to_string()){
            return true;
        }
        else {
            return false;
        }

    }
    
    pub fn add_op(&mut self) -> bool {
        if self.expect_str("+".to_string()) || self.expect_str("-".to_string()) {
            return true;
        }
        else {
            return false;
        }

    }
    
    pub fn mult_op(&mut self) -> bool {
        if self.expect_str("*".to_string()) || self.expect_str("/".to_string()) {
            return true;
        }
        else {
            return false;
        }
        
    }

    pub fn out_token(&mut self) -> Option<Token> {
        self.cur_token_num = self.cur_token_num + 1;
        if self.num_parsed <= self.cur_token_num {
            return None;
        }
        else {
            return Some(self.parsed_token_stream.remove(0));
        }
    }

    pub fn peek_ahead_token(&mut self) -> Option<i32> {
        if self.num_parsed <= self.cur_token_num + 1 {
            return None;
        }
        else {
            return Some(self.parsed_token_stream.get(0).unwrap().get_line_number());
        }
    }

}