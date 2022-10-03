use std::io::stdin;
use std::io::{stdout, Write};

fn main() {
    loop {
        println!("select action. ");
        print!(">> ");
        stdout().flush().unwrap();

        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .expect("failed to read input");

        match buffer.as_str().trim() {
            "exit" => {
                println!("good bye!");
                break;
            }
            "1" | "2" | "3" | "4" | "5" | "6" | "l" | "r" => {
                println!("play card!");
            }
            "d" => {
                println!("draw!");
            }
            "p" => {
                println!("pass!");
            }
            _ => {}
        }
    }
}
