use std::{fs::File, io::{Read, Write}};

enum Lexems {
    Number(i64),
    String(String),
    Delimeter(char),
    EOF(char),
    NotLexem(char)
}

enum States {
    Digit,
    Letter,
    Delim,
}

fn w_num_lexem(digit_buf: & mut Vec<char>, mut parser_input_file:& File) -> Result<String, String>
{
    if digit_buf.len() != 0
    {
        let mut full_lexema = "Number-".to_string();
        full_lexema = full_lexema + &buf_to_lexem(&digit_buf) + "\n";
        
        digit_buf.clear();
    
        match parser_input_file.write(full_lexema.as_bytes()) {
            Err(_) => return Err("Error with save lexem into file".to_string()),
            Ok(_) => ()
        };
    }
    
    Ok(" ".to_string())
}

fn w_str_lexem(letter_buf: & mut Vec<char>, mut parser_input_file:& File) -> Result<String, String>
{
    if letter_buf.len() != 0 
    {
        let mut full_lexema = "String-".to_string();
        full_lexema = full_lexema + &buf_to_lexem(&letter_buf) + "\n";
        
        letter_buf.clear();
    
        match parser_input_file.write(full_lexema.as_bytes()) {
            Err(_) => return Err("Error with save lexem into file".to_string()),
            Ok(_) => ()
        }
    }

    Ok(" ".to_string())
}

fn w_delim_lexem(ch:& char, mut parser_input_file:& File) -> Result<String, String>
{                        
    let mut full_lexema = "Delimeter-".to_string();
    full_lexema.push(*ch);
    full_lexema = full_lexema + "\n";

    match parser_input_file.write(full_lexema.as_bytes()) {
        Err(_) => return Err("Error with save lexem into file".to_string()),
        Ok(_) => Ok(" ".to_string())
    }
}

fn buf_to_lexem(char_buf: & Vec<char>) -> String
{
    let mut output_lexem:String = "".to_string();

    for ch in char_buf {
       output_lexem.push(*ch);
    }

    output_lexem
}

pub fn file_lexerize(file:&str, lexem_file:&str) -> Result<String, String>
{
    let mut lexer_input_file = match File::open(file) {
        Ok(descript) => descript,
        Err(_) => return Err("Cannot open the code file ".to_string() + file)
    };
    
    let parser_input_file = match File::create(lexem_file) {
        Ok(descript) => descript,
        Err(_) => return Err("Cannot open the saves lexem file ".to_string() + file)
    };

    let cur_char:&mut[u8] = &mut[1];
    let mut cur_state = States::Delim;

    let mut digit_buf: Vec<char> = Vec::new();
    let mut letter_buf: Vec<char> = Vec::new();

    let mut count_byte_read:usize = 0;

    loop
    {
        count_byte_read = match lexer_input_file.read(cur_char) {
            Err(_) => return Err("Can't reads byte out of file".to_string()),
            Ok(count) => count
        };

        if count_byte_read == 0
        {
            match w_num_lexem(&mut digit_buf, &parser_input_file) {
                Err(err) => return Err(err),
                Ok(_) => ()
            }

            match w_str_lexem(&mut letter_buf, &parser_input_file) {
                Err(err) => return Err(err),
                Ok(_) => ()
            }
            
            println!("Lexers work done");
            return Ok("./parser_input.txt".to_string());
        }
            

        let ch = cur_char[0] as char; // Преобразуем байт в char

        match ch {
            ';' => {
                if digit_buf.len() != 0
                {
                    match w_num_lexem(&mut digit_buf, &parser_input_file) {
                        Err(err) => return Err(err),
                        Ok(_) => ()
                    }
                }

                if letter_buf.len() != 0
                {
                    match w_str_lexem(&mut letter_buf, &parser_input_file) {
                        Err(err) => return Err(err),
                        Ok(_) => ()
                    }
                }

                return Ok("./parser_input.txt".to_string());
            },
            ch if ch == '|' || ch == ':' => {
                match cur_state {
                    States::Digit => {
                        match w_num_lexem(&mut digit_buf, &parser_input_file) {
                            Err(err) => return Err(err),
                            Ok(_) => ()
                        }

                        cur_state = States::Delim;
                        
                        match w_delim_lexem(&ch, &parser_input_file) {
                            Err(err) => return Err(err),
                            Ok(_) => ()
                        }
                    },
                    States::Letter => {
                        match w_str_lexem(&mut letter_buf, &parser_input_file) {
                            Err(err) => return Err(err),
                            Ok(_) => ()
                        }

                        cur_state = States::Delim;
                        
                        match w_delim_lexem(&ch, &parser_input_file) {
                            Err(err) => return Err(err),
                            Ok(_) => ()
                        }
                    },
                    _ => {
                        cur_state = States::Delim;
                        
                        match w_delim_lexem(&ch, &parser_input_file) {
                            Err(err) => return Err(err),
                            Ok(_) => ()
                        }
                    }
                }
            },
            ch if ch.is_ascii_alphabetic() =>
            {
                match cur_state {
                    States::Digit => {
                        match w_num_lexem(&mut digit_buf, &parser_input_file) {
                            Err(err) => return Err(err),
                            Ok(_) => ()
                        }

                        cur_state = States::Letter;
                        letter_buf.push(ch);
                    },
                    _ => {
                        cur_state = States::Letter;
                        letter_buf.push(ch);
                    }
                }
            },
            ch if ch.is_digit(10) =>
            {
                match cur_state {
                    States::Letter => {
                        match w_str_lexem(&mut letter_buf, &parser_input_file) {
                            Err(err) => return Err(err),
                            Ok(_) => ()
                        }

                        cur_state = States::Digit; 
                        digit_buf.push(ch);
                    },
                    _ => {
                        cur_state = States::Digit;
                        digit_buf.push(ch);
                    }
                }
            },
            _ => {
                match w_num_lexem(&mut digit_buf, &parser_input_file) {
                    Err(err) => return Err(err),
                    Ok(_) => ()
                }

                match w_str_lexem(&mut letter_buf, &parser_input_file) {
                    Err(err) => return Err(err),
                    Ok(_) => ()
                }

                return Err("Unrecognized lexem".to_string());
            }
        }
    }
}