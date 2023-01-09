use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use argh::FromArgs;
use color_eyre::eyre::Result;


#[derive(FromArgs)]
#[argh(description = r#"


 ________  ________                 ________  ________  _______   ________   
|\   __  \|\   ____\               |\   ____\|\   __  \|\  ___ \ |\   __  \  
\ \  \|\  \ \  \___|_  ____________\ \  \___|\ \  \|\  \ \   __/|\ \  \|\  \ 
 \ \   _  _\ \_____  \|\____________\ \  \  __\ \   _  _\ \  \_|/_\ \   ____\
  \ \  \\  \\|____|\  \|____________|\ \  \|\  \ \  \\  \\ \  \_|\ \ \  \___|
   \ \__\\ _\ ____\_\  \              \ \_______\ \__\\ _\\ \_______\ \__\   
    \|__|\|__|\_________\              \|_______|\|__|\|__|\|_______|\|__|   
              \|_________|                                                    
                                                                              

A simple grep alternative written in Rust.

By CM-IV <chuck@civdev.xyz>
"#)]
struct Args {
    #[argh(positional)]
    pattern: String,

    #[argh(positional)]
    path: String,

}

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    let pattern = &args.pattern;
    let path = Path::new(&args.path);

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                grep(pattern, &path)?;
            }
        }
    } else if path.is_file() {
        grep(pattern, path)?;
    } else {
        eprintln!("Error: {} is not a file or a directory", args.path);
        std::process::exit(1);
    }

    Ok(())
}

fn grep(pattern: &str, path: &Path) -> Result<(), io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(pattern) {
            println!("{}:{}", path.display(), line_number + 1);
            println!("{}", line);
        }
    }

    Ok(())
}
