use std::{
    env,
    fs::File,
    io::Write,
    io::{self, Read},
    path::Path,
};

extern crate type_lib;
use type_lib::{
    generator::{gleam, Generator},
    parser::Parser,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file_path = match file_path.strip_suffix(".type") {
        Some(f) => f,
        None => file_path,
    };

    let mut file = File::open(format!("{file_path}.type"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut parser = Parser::new(&contents);
    let mut generator = gleam::GleamTypeGenerator::new();
    for ty in parser.parse() {
        generator.add_type(&ty);
    }

    let result = generator.generate();
    for ty in result {
        let output_file_name = format!("{}.gleam", ty.name.to_lowercase());
        let mut output_file = File::create(&output_file_name)?;

        writeln!(output_file, "{}", ty.content)?;
        println!("Decoder written to {}", output_file_name);
    }

    Ok(())
}
