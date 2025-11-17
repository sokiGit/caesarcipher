use std::process::exit;
#[derive(PartialEq)]
#[derive(Debug)]
enum ParseState {
    SetRotation,
    Unnamed,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Mode {
    Encipher,
    Decipher,
}

fn rotate_char(char: char, rot: u8) -> char {
    if char >= 'A' && char <= 'Z' {
        // Uppercase
        let base = 'A' as u8;
        let rotated = (char as u8 - base + rot) % 26 + base;
        rotated as char
    } else if char >= 'a' && char <= 'z' {
        // Lowercase
        let base = 'a' as u8;
        let rotated = (char as u8 - base + rot) % 26 + base;
        rotated as char
    } else {
        // Special character or number
        char
    }
}


fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    let mut mode: Mode = Mode::Encipher;
    let mut rotation: u8 = 0;
    let mut text_input: String = String::new();
    let mut print_help: bool = false;

    let mut parse_state: ParseState = ParseState::Unnamed;

    for argument in args {
        match argument.as_str() {
            // Flags
            "-d" | "--decipher" => {
                if parse_state != ParseState::Unnamed {
                    println!("Argument mismatch, maybe you forgot to specify a value");
                    exit(1);
                }
                mode = Mode::Decipher;
            }
            "-r" | "--set-rotation" => {
                if parse_state != ParseState::Unnamed {
                    println!("Argument mismatch, maybe you forgot to specify a value");
                    exit(1);
                }
                parse_state = ParseState::SetRotation;
            }
            "-h" | "--help" => {
                print_help = true;
            }
            _ => {
                // Values
                match parse_state {
                    ParseState::Unnamed => {
                        // The plaintext/ciphertext
                        text_input = argument;
                        break;
                    }
                    ParseState::SetRotation => {
                        match argument.as_str().parse::<isize>() {
                            Ok(num) => {
                                rotation = (num % 26) as u8;
                            }
                            _ => {
                                println!("Can't parse argument '{argument}' as integer.");
                                exit(2);
                            }
                        }

                        parse_state = ParseState::Unnamed;
                    }
                }
            }
        }
    }

    if mode == Mode::Decipher {
        rotation = 26 - rotation;
    }

    // Commands
    // -h, --help
    if print_help {
        println!("Usage:");
        println!("  -d, --decipher\tDecipher mode (Default behavior is encipher)");
        println!("  -r [int], --set-rotation [int]\tSets the rotation (default: 0)");
        println!("  -h, --help\tPrint this help message");
    }

    // En/Decipher
    let mut result: String = String::new();

    for char in text_input.as_str().chars() {
        result.push(rotate_char(char, rotation));
    }

    match mode {
        Mode::Encipher => {
            println!("Detected original: {text_input}\nCiphertext: {result}");
        }
        Mode::Decipher => {
            println!("Detected ciphertext: {text_input}\nOriginal: {result}");
        }
    }
}
