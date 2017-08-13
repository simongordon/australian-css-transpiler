extern crate rand;
use std::env;
//use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::fs::File;
use std::path::Path;
//use std::time::SystemTime;
use rand::Rng;

//use regex::Regex;

fn main() {
    print_random(&["g'day", "'scarn on"]);
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        if &args[1] == "--help" || &args[1] == "-h" {
            println!("Help");
        }
        else if args.len() > 2 {
            let input_path = &args[1];
            let output_path = &args[2];
            println!("{} {}", input_path, output_path);
            transpile(input_path, output_path);
        }
    }
    let this_file = file!();
    println!("defined in file: {}", this_file);
    print_random(&["cheers mate", "onya mate", "true blue"]);
}

fn print_random(array: &[&str]) {
    let index = rand::thread_rng().gen_range(0, array.len());
    println!("{}", array[index]);
}

fn transpile(input_path: &str, output_path: &str) {
    assert!(Path::new(input_path).exists());
    //assert!(Path::new(output_path).exists());
    //let mut input_file = File::open(input_path).unwrap();
    let mut input_file = File::open(input_path).expect("Unable to open");
    let mut output_file = match File::open(output_path) {
        Ok(file) => file,
        Err(_) => File::create(output_path).unwrap(),
    };
    let mut buffer = String::new();
    input_file.read_to_string(&mut buffer).expect("failed to read");
    //println!("Buffer: {}, {}", buffer, buffer.len());

    //let re = Regex::new(r"colour").unwrap();
    //let result = re.replace_all(buffer, "color");
    let result = str::replace(&buffer, "colour", "color");
    println!("{}", result); // => "xxxxx xxxxx!"
    output_file.write(result.as_bytes()).expect("failed to write");

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_transpile() {
        //transpile("/src/input.css", "/src/output.css");
        //transpile("/src/input.css", "/src/output.css");
        //transpile("\\src\\input.css", "\\src\\output.css");
    }
}
