use std::fmt::{Display, Formatter};

use clap::Parser;
use colorized::{colorize_this, Colors};
use regex::Regex;

#[derive(Parser)]
pub struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

impl Display for Cli {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pattern: {}, path: {}",
            self.pattern, self.path.display()
        )
    }
}

fn main() {
    let args = Cli::parse();
    println!("{}", &args);

    let p = &args.pattern;
    let r = Regex::new(p).unwrap();

    let mut content = std::fs::read_to_string(&args.path).expect("could not read file");

    // let dir = std::fs::read_dir(&args.path).unwrap();

    process(&r, &mut content);
}

/// 处理每个文件
fn process(r: &Regex, content: &mut String) {


    let mut num: i64 = 0;
    for line in content.lines() {
        num += 1;
        if r.is_match(line) {
            // get start index
            let start = r.find(line).unwrap().start();
            let colored_start = colorize_this(start.to_string(), Colors::RedFg);

            //  colored matched font
            let captures = r.captures(line).unwrap();
            let c = captures.get(0).unwrap().as_str();
            let color_str = colorized::colorize_this(c, Colors::GreenFg);
            let cow = r.replace(line, color_str);

            println!("{}:{} {}", num, colored_start, cow);
        }
    }
}



