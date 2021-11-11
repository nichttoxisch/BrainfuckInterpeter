use std::env::args;
use std::fs;
use std::num::Wrapping;
use std::process::exit;

const BUFFER_SIZE: usize = 3001;

fn brainfuck(code: String) -> String {
    let prog = code.as_bytes();
    let mut buf = [0u8; BUFFER_SIZE];
    let mut p = 0 as usize;
    let mut b = 0 as usize;
    let mut output: String = String::new();

    while p < prog.len() && prog[p] != '!' as u8 {
        match prog[p] as char {
            '+' => {
                while prog[p] == '+' as u8 {
                    // buf[b] += 1;
                    buf[b] = (Wrapping(buf[b]) + Wrapping(1u8)).0;
                    p += 1;
                }
                p -= 1;
            }
            '-' => {
                while prog[p] == '-' as u8 {
                    // buf[b] -= 1;
                    buf[b] = (Wrapping(buf[b]) - Wrapping(1u8)).0;
                    p += 1;
                }
                p -= 1;
            }
            '>' => {
                while prog[p] == '>' as u8 {
                    b += 1;
                    p += 1;
                }
                p -= 1;
            }
            '<' => {
                while prog[p] == '<' as u8 {
                    b -= 1;
                    p += 1;
                }
                p -= 1;
            }
            '[' => {
                if buf[b] != 0 {
                    p += 1;
                    continue;
                }
                let mut inner = 0;
                let mut keep = true;
                while keep {
                    p += 1;

                    if String::from_utf8_lossy(&prog[p..p + 3]) == "[-]" {
                        buf[b] = 0;
                        p += 2;
                    } else {
                        if prog[p] == '[' as u8 {
                            inner += 1;
                        }

                        if prog[p] == ']' as u8 {
                            if inner > 0 {
                                inner -= 1;
                            } else {
                                keep = false;
                            }
                        }
                    }
                }
            }
            ']' => {
                if buf[b] == 0 {
                    p += 1;
                    continue;
                }

                p -= 1;
                let mut inner = 0;
                let mut keep = true;
                while keep {
                    if prog[p] == ']' as u8 {
                        inner += 1;
                    }

                    if prog[p] == '[' as u8 {
                        if inner > 0 {
                            inner -= 1;
                        } else {
                            p += 1;
                            keep = false;
                        }
                    }
                    p -= 1
                }
            }
            '.' => {
                output += &String::from(buf[b] as char);
                print!("{}", buf[b] as char);
            }
            ',' => println!("ERROR: getchar not implemented"),
            _ => print!("ERROR: Unknows character"),
        }
        // print!("{} ({})\t", p, prog[p] as char);

        // for i in 0..20 {
        //     if i == b || i == b + 1 {
        //         print!("|");
        //     }
        //     if i == b {
        //         print!("{}", buf[i]);
        //     } else {
        //         print!(" {} ", buf[i])
        //     }
        // }
        // print!("\n");

        p += 1;
    }

    return output;
}

fn main() {
    let path: Vec<String> = args().collect();

    if path.len() <= 1 {
        println!("USAGE: brainfuck_interpeter [PATH TO FILE]");
        exit(0);
    }

    let mut code = fs::read_to_string(path[1].as_str()).expect("Provided file path does not exist");
    code = code.replace("\n", "").replace("\t", "").replace(" ", "");

    brainfuck(code);
    // println!("output: {}", out);
}
