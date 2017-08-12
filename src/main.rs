use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        if &args[1] == "--help" || &args[1] == "-h" {
            println!("Help");
        }
        else if args.len() > 2 {
            let input_path = &args[0];
            let output_path = &args[1];
            println!("{} {}", input_path, output_path);
            transpile(input_path, output_path);
        }
    }
    let this_file = file!();
    println!("defined in file: {}", this_file);
}

fn transpile(input_path: &str, output_path: &str) {
    assert!(Path::new(input_path).exists());
    //assert!(Path::new(output_path).exists());
    let mut input_file = File::open(input_path);
    let mut output_file = File::open(output_path);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_transpile() {
        //transpile("/src/input.css", "/src/output.css");
        //transpile("/src/input.css", "/src/output.css");
        transpile("\\src\\input.css", "\\src\\output.css");
    }
}
