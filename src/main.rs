pub mod lexer;
pub mod parser;

fn main() {
    let file_code_path = "./src/code.txt";
    let file_lexem_path = "./src/lexem.txt";

    let result:Result<String, String> = lexer::file_lexerize(file_code_path, file_lexem_path);

    let lexems_path = match result
    {
        Ok(path) => path,
        Err(error) => { 
            println!("{}", error);
            return ()
        }
    };
}
