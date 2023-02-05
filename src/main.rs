use parser::{parse_args, read_settings_file};
use randomizer::SensitivityRandomizer;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

pub mod parser;
pub mod randomizer;

fn main() {
    let (min, max) = parse_args();
    let settings = read_settings_file();

    let sens_randomizer = SensitivityRandomizer::new(min, max);
    let generated_sensitivity = sens_randomizer.generate_random_sensitivity();
    let new_settings = update_settings(settings, generated_sensitivity);

    create_new_settings(&new_settings);

    println!(
        "{}",
        format!("Generated sensitivity: {generated_sensitivity:.2}",)
    );
}

fn update_settings(settings: Vec<String>, new_sensitivity: f32) -> Vec<String> {
    settings
        .iter()
        .map(|line| {
            if !line.starts_with("mouse_sensitivity") {
                format!("{line}\n")
            } else {
                format!("mouse_sensitivity \"{new_sensitivity:.2}\"\n")
            }
        })
        .collect()
}

fn create_new_settings(new_settings: &Vec<String>) {
    match File::create("settings.cfg") {
        Ok(mut file) => {
            for line in new_settings {
                file.write_all(line.as_bytes()).unwrap();
            }
        }

        Err(_) => {
            eprintln!("Could not create settings.cfg!");
            exit(0);
        }
    }
}
