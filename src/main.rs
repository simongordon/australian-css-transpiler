use std::env;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::fs::File;
use std::path::Path;

//use regex::Regex;

fn main() {
    println!("g'day");
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
    println!("onya mate");
}

fn transpile(input_path: &str, output_path: &str) {
    assert!(Path::new(input_path).exists());
    //assert!(Path::new(output_path).exists());
    //let mut input_file = File::open(input_path).unwrap();
    let mut input_file = File::open(input_path).expect("Unable to open");
    let mut output_file = match File::open(output_path) {
        Ok(file) => file,
        Err(file) => File::create(output_path).unwrap(),
    };
    let mut buffer = String::new();
    input_file.read_to_string(&mut buffer);
    //println!("Buffer: {}, {}", buffer, buffer.len());

    //let re = Regex::new(r"colour").unwrap();
    //let result = re.replace_all(buffer, "color");
    let result = str::replace(&buffer, "colour", "color");
    println!("{}", result); // => "xxxxx xxxxx!"
    output_file.write(result.as_bytes());

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
