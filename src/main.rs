mod character_stream;

mod token;

mod scanner;
use scanner::*;

mod parser;
use parser::*;

use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

use std::env;

fn main() -> std::io::Result<()>{

	let args: Vec<String> = env::args().collect();
	let command1 = args[1].clone(); 

	let mut scanner = Scanner::new(&command1);

	while scanner.check_more_available() {
		scanner.lex();
		scanner.get_non_blank();
	}

	let mut parser = Parser::new(scanner);
	parser.program();
	println!("num parsed: {}", parser.get_num_parsed());

	let mut file = File::create("out.xhtml")?;
	let mut data = String::from("<!DOCTYPE html PUBLIC ");
	data.push_str("\"-//W3C//DTD XHTML 1.0 Transitional//EN\"");
	data.push_str(" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\"");
	data.push_str(">\n");
	data.push_str("<html xmlns=");
	data.push_str("\"http://www.w3.org/1999/xhtml\" ");
	data.push_str("xml:lang=\"en\">\n");
	data.push_str("<head>\n");
	data.push_str("<title>\n");
	data.push_str("X Formatted file</title>\n");
	data.push_str("</head>\n");
	data.push_str("<body bgcolor=\"navy\" text=\"yellow\" link=\"yellow\" vlink=\"yellow\">\n");
	data.push_str("<font face=\"Courier New\">\n");
	//loop, add to string add to string as needed. 

	let mut cur_tok = parser.out_token();

	let first_half_str_color = "<font color=";
	let mut str_color;

	let color_lookup = HashMap::from([
		("Function".to_string(), "\"orange\">"),
		("Variable".to_string(), "\"yellow\">"),
		("FloatConstant".to_string(), "\"aqua\"><b>"),
		("IntConstant".to_string(), "\"aqua\"><b>"),
		("Operator".to_string(), "\"white\"><b>"),
		("Keyword".to_string(), "\"white\"><b>"),
	]);

	let bold_lookup = HashMap::from([
		("Function".to_string(), false),
		("Variable".to_string(), false),
		("FloatConstant".to_string(), true),
		("IntConstant".to_string(), true),
		("Operator".to_string(), true),
		("Keyword".to_string(), true),
	]);

	let mut tab_count= 0;
	let mut seen_new_line = false;


	while let Some(ref i) = cur_tok {
		if i.get_text().eq("}") && tab_count != 0 {
			tab_count = tab_count - 1;	
		}

		if tab_count >= 1 && seen_new_line {
			for _x in 0..tab_count {
				if tab_count == 1 {
					data.push_str("&nbsp;&nbsp; &nbsp;");
				}
				if tab_count > 1 {
					data.push_str("&nbsp; &nbsp;");
				}
			}
			seen_new_line = false;
		}

		data.push_str(first_half_str_color);
		str_color = color_lookup[i.get_type().as_str()];
		data.push_str(str_color);
		data.push_str(i.get_text().clone());
		if bold_lookup[i.get_type().as_str()] {
			data.push_str("</b></font> ");
		}
		else {
			data.push_str("</font> ");
		}
		let peek_next = parser.peek_ahead_token();
		if let Some(j) = peek_next {
			if j >= i.get_line_number() + 1 {
				data.push_str("<br />\n");
				seen_new_line = true;
			}
		}

		if i.get_text().eq("{") {
			tab_count = tab_count + 1;
		}
		
	
		cur_tok = parser.out_token();
	}

	data.push_str("</font>\n");
	data.push_str("</body>\n");
	data.push_str("</html>\n");

	//have: tokens in vector stream. 
	//write!(file, "{}", data);
	Ok(())
	
}


