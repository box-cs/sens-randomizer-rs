use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;

pub fn parse_args() -> (f32, f32) {
    let args: Vec<f32> = env::args()
        .skip(1)
        .take(2)
        .map(|x| x.parse::<f32>().unwrap())
        .collect();

    let min = *args.get(0).unwrap_or(&0.90);
    let max = *args.get(1).unwrap_or(&1.30);

    if min > max {
        eprintln!("Minimum range cannot be greater than max!");
        exit(0);
    }

    (min, max)
}

pub fn read_settings_file() -> Vec<String> {
    read_lines("./settings.cfg")
        .unwrap()
        .map(|line| line.unwrap_or(String::from("")))
        .collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
