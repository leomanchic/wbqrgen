#![allow(unused)]
use qrcode_generator::QrCodeEcc;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use clap::Parser;





#[derive(Parser,Debug)]
#[command(author, version, about="CLI Application for qr-code generation")]
struct CLI{
    /// Determine the desired extension
    #[clap(short = 'e', long = "extention")]
    pattern: String,
    /// The path to the file to read from
    #[clap(short = 'f', long = "file")]
    path: std::path::PathBuf,
    #[arg(short = 's', long = "size",default_value_t = 500)]
    ///Size of an output image in pixels
    size: usize,
}

//чтение строчек из файла , каждая строка - отдельный qr-code
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = match  File::open(filename){
        Ok(f) => Ok(io::BufReader::new(f).lines()),
        Err(_) => panic!("Error while reading file!!!"),
    };
    file
}

fn processing(size: &usize, path: &std::path::PathBuf,pattern: &str){
    if let Ok(lines) = read_lines(&path) {
        // Получает итератор, который возвращает Option
        for line in lines {
            if let Ok(product) = line {
                //Определить расширение файла
                let mut string = product[..].to_string();
                
                match &pattern[..] {
                    "png" => {
                        &string.push_str(".png");
                        qrcode_generator::to_png_to_file(product, QrCodeEcc::Low, *size, &string).unwrap()
                    },
                    "svg" => {
                        &string.push_str(".svg");
                        qrcode_generator::to_svg_to_file(&product, QrCodeEcc::Low,*size,None::<&str>, &string).unwrap()
                    },
                    other => panic!("Wrong extention!!!"),
                }
            }      
        }   
    }

}


fn main() {
    let args = CLI::parse();
    processing(&args.size, &args.path, &args.pattern);
}