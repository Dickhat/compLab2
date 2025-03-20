pub mod lexer;
pub mod parser;

use std::fs::File;
use std::io::{self, Read};


fn main() {
    let file_code_path = "./src/code.txt";
    let file_lexem_path = "./src/lexem.txt";

    let mut result:Result<String, String> = lexer::file_lexerize(file_code_path, file_lexem_path);

    let lexems_path = match result
    {
        Ok(path) => path,
        Err(error) => { 
            println!("{}", error);
            return ()
        }
    };

    result = parser::syntax_parse(&lexems_path);
    
    match result {
        Ok(_) => println!("Lexer and parsers work done"),
        Err(text) => println!("{}", text)
    }
}
