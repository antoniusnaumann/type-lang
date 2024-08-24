use std::{
    env,
    fs::File,
    io::Write,
    io::{self, Read},
};

extern crate type_lib;
use type_lib::{
    generator::{gleam, rust, Generator},
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

    let generators: Vec<Box<dyn Generator>> = vec![
        gleam::GleamTypeGenerator::boxed(),
        rust::RustTypeGenerator::boxed(),
    ];

    for mut generator in generators {
        let mut parser = Parser::new(&contents);
        for ty in parser.parse() {
            generator.add_type(&ty);
        }

        let ext = generator.file_extension();
        let result = generator.types();
        for ty in result {
            let output_file_name = format!("{}.{ext}", ty.name.to_lowercase());
            let mut output_file = File::create(&output_file_name)?;

            writeln!(output_file, "{}", ty.content)?;
            println!("Decoder written to {}", output_file_name);
        }
    }

    Ok(())
}
