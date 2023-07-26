#![allow(unused)]
use qrcode_generator::QrCodeEcc;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use clap::Parser;

#[derive(Parser)]
struct CLI{
    /// The pattern to look for
    #[arg(short = 'e', long = "extention")]
    pattern: String,
    /// The path to the file to read
    #[arg(short = 'f', long = "file")]
    path: std::path::PathBuf,
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = match  File::open(filename){
        Ok(f) => Ok(io::BufReader::new(f).lines()),
        Err(_) => panic!("Error while reading file"),
    };
    file
}


fn main() {
    let args = CLI::parse();
    if let Ok(lines) = read_lines(&args.path) {
        // Получает итератор, который возвращает Option
        for line in lines {
            if let Ok(product) = line {
                let mut string = product[..].to_string();
                if &args.pattern == "svg"{
                    &string.push_str(".svg");
                    qrcode_generator::to_svg_to_file(&product, QrCodeEcc::Low, 500,None::<&str>, &string).unwrap();
                };
                if &args.pattern == "png"{
                    &string.push_str(".png");
                    qrcode_generator::to_png_to_file(product, QrCodeEcc::Low, 500, string).unwrap();
                };
                
                
            }      
        }   
    }
      
}