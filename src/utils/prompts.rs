// Handles CLI prompts and arg parsing
// TODO: add cute UI :3

use std::{
    io::{stdin, stdout, Write},
    path::Path,
    str::FromStr,
};

/// Struct containing script parameters, collected via CLI at runtime
pub struct Options {
    pub input_path: String,
    pub output_path: String,
    pub k: usize,
    pub iterations: usize,
    pub mode: Mode,
}

/// enum designating how to treat a given CLI arg
enum Prompt {
    Path,
    Safe,
    Num,
    Mode,
}

// Enum designating whether program should output a stylized image or centroid values as palette
pub enum Mode {
    Palette,
    Image,
}

impl Options {
    pub fn new() -> Options {
        let prompts = [
            ("Enter input path: ", Prompt::Path),
            ("Enter output path: ", Prompt::Safe),
            ("Enter K value: ", Prompt::Num),
            ("Enter # of iterations: ", Prompt::Num),
            ("Create palette? (y/n): ", Prompt::Mode),
        ];

        // group prompt text and enum together, then call prompt fn
        let opts = prompts
            .into_iter()
            .map(|(pr, pa)| prompt(pr, pa))
            .collect::<Vec<String>>();

        // return validated result
        Options {
            input_path: opts[0].clone(),
            output_path: opts[1].clone(),
            k: usize::from_str(&opts[2]).expect("Cannot parse K value!"),
            iterations: usize::from_str(&opts[3]).expect("Cannot parse iterations!"),
            mode: match opts[4].to_lowercase().as_str() {
                "y" | "yes" => Mode::Palette,
                _ => Mode::Image,
            },
        }
    }
}

// Input parsing and validation
fn prompt(line: &str, parse: Prompt) -> String {
    let mut first = true;
    let mut input = String::new();

    let out = stdout();
    let mut handle = out.lock();
    loop {
        // determine whether first attempt
        if first {
            println!("{line}");
        } else {
            handle.write_all(b"invalid input \"").unwrap();
            handle.write_all(input.trim().as_bytes()).unwrap();
            handle.write_all(b"\" please try again \n").unwrap();
            handle.flush().unwrap();
            input.clear();
        }

        stdin().read_line(&mut input).unwrap();

        // Match prompt with enum to determine how its parsed
        // TODO: wrestle with lifetimes, make these return Option<&str>
        let parse_fn = match parse {
            Prompt::Path => |p: &str| {
                if Path::new(&p.trim()).exists() {
                    Some(p.to_string())
                } else {
                    None
                }
            },
            Prompt::Safe => |s: &str| Some(s.to_string()),
            Prompt::Num => |n: &str| {
                if n.trim().chars().all(|c| c.is_ascii_digit()) {
                    Some(n.to_string())
                } else {
                    None
                }
            },
            Prompt::Mode => |m: &str| match m.trim().to_lowercase().as_str() {
                "n" | "no" | "y" | "yes" => Some(m.to_string()),
                _ => None,
            },
        };

        // if parse is succesful, return trimmed line, otherwise prompt again
        if let Some(out) = parse_fn(&input) {
            return out.replace('\n', "");
        } else {
            first = false;
            continue;
        }
    }
}
