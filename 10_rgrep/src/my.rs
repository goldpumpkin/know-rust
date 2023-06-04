use std::fmt::{Display, Formatter};
use std::thread;

use clap::Parser;
use colorized::{colorize_this, Colors};
use glob::glob;
use regex::Regex;
use tokio::runtime::Builder;

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

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    println!("{}", &args);

    // 匹配内容
    let p = &args.pattern;
    let r = Regex::new(p).unwrap();

    // 读取文件
    let x = args.path.to_str().unwrap();
    let paths = glob(x).unwrap();

    let tasks = Builder::new_multi_thread()
        .thread_name("goldpumpkin-worker")
        .build().unwrap();

    let mut result = Vec::new();
    for g in paths {
        let r_clone = r.clone();
        let per_path = g.unwrap();
        println!("{}:", &per_path.display());
        let handle = tasks.spawn(async move {
            println!("thread:{}, start handle one path..", thread::current().name().unwrap());
            println!("{}:", &per_path.display());
            let mut content = std::fs::read_to_string(&per_path).expect("could not read file");

            process(&r_clone, &mut content);
        });
        &result.push(handle);
        println!("------------file split------------");
    }

    for t in result {
        t.await.unwrap();
    }

    tasks.shutdown_background();
}

/// process per file
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



