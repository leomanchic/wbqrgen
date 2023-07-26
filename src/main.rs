use qrcode_generator::QrCodeEcc;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use clap::Parser;



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = match  File::open(filename){
        Ok(f) => Ok(io::BufReader::new(f).lines()),
        Err(_) => panic!("Error while reading file"),
    };
    file
}


fn main() {
    if let Ok(lines) = read_lines("./wbqr.txt") {
        // Получает итератор, который возвращает Option
        for line in lines {
            if let Ok(product) = line {
                let mut string = product[..].to_string();
                string.push_str(".png");
                qrcode_generator::to_png_to_file(product, QrCodeEcc::Low, 500, string).unwrap();
            }      
        }   
    }
      
}