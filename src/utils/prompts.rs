use std::{
    io::stdin,
    path::Path,
    str::FromStr,
    
};

enum Prompt {
    Path,
    Safe,
    Num,
    Mode,
}

pub struct Options {
    pub input_path: String,
    pub output_path: String,
    pub k: u16,
    pub iterations: u16,
    pub mode: String,
}

impl Options {
    pub fn new() -> Options {
        let prompts = [
            "Enter input path: ",
            "Enter output path: ",
            "Enter K value: ",
            "Enter # of iterations: ",
            "Create palette? (y/n): ",
        ];

        let parse = [
            Prompt::Path,
            Prompt::Safe,
            Prompt::Num,
            Prompt::Num,
            Prompt::Mode,
        ];

        let opts = prompts
            .iter()
            .zip(parse)
            .map(|(pr, pa)| prompt(pr, pa))
            .collect::<Vec<String>>();

        Options {
            input_path: opts[0].clone(),
            output_path: opts[1].clone(),
            k: u16::from_str(&opts[2]).unwrap(),
            iterations: u16::from_str(&opts[3]).unwrap(),
            mode: opts[4].clone(),
        }
    }
}

fn prompt(line: &str, parse: Prompt) -> String {
    let mut first = true;
    let mut input = String::new();
    loop {
        if first {
            println!("{line}")
        } else {
            println!("invalid input \n \"{input}\" \n please try again");
            input.clear();
        }

        stdin().read_line(&mut input).unwrap();
        let parse_fn = match parse {
            Prompt::Path => |p: &str| -> bool { Path::new(&p.trim_matches('\n')).exists() },
            Prompt::Safe => |_: &str| true,
            Prompt::Num => |n: &str| -> bool { u16::from_str(n.trim()).is_ok() },
            Prompt::Mode => |m: &str| -> bool { m.trim() == "y" || m.trim() == "n" },
        };

        if parse_fn(&input) {
            return input.trim().to_string();
        } else {
            first = false;
            continue;
        }
    }
}
