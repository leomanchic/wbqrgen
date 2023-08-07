use wbqrgen::CLI;
use clap::Parser;
fn main() {
    let args = CLI::parse();
    wbqrgen::processing(&args.size, &args.path, &args.pattern);
}