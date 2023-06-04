use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Stdout, Write};
use std::ops::Range;
use std::path::Path;

use clap::Parser;
use colored::*;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

use crate::error::GrepError;

mod error;

/// 定义类型，这样，在使用时可以简化复杂类型的书写
pub type StrategyFn<W, R> = fn(&Path, BufReader<R>, &Regex, &mut W) -> Result<(), GrepError>;

#[derive(Parser, Debug)]
#[command(name = "rgrep")]
#[command(author = "Gold Huang <goldhuang24@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Simple Grep", long_about = None)]
pub struct GrepConfig {
    pattern: String,
    glob: String,
}

impl GrepConfig {
    pub fn match_with_default_strategy(&self) -> Result<(), GrepError> {
        self.match_with(default_strategy)
    }

    pub fn match_with(&self, strategy: StrategyFn<Stdout, File>) -> Result<(), GrepError> {
        let regex = Regex::new(&self.pattern).unwrap();
        let files: Vec<_> = glob::glob(&self.glob).unwrap().collect();
        files.into_par_iter().for_each(|v| {
            if let Ok(filename) = v {
                if let Ok(file) = File::open(&filename) {
                    let reader = BufReader::new(file);
                    let mut stdout = std::io::stdout();

                    let x = filename.as_path();
                    if let Err(e) = strategy(x, reader, &regex, &mut stdout) {
                        println!("Internal error: {:?}", e);
                    }
                }
            }
        });

        Ok(())
    }
}


pub fn default_strategy<W: Write, R: Read>(
    path: &Path,
    reader: BufReader<R>,
    pattern: &Regex,
    writer: &mut W,
) -> Result<(), GrepError> {
    let matches: String = reader.lines()
        .enumerate()
        .map(|(lineno, line)| {
            line.ok().map(|line|
                {
                    pattern.find(&line)
                        .map(|m| format_line(&line, lineno + 1, m.range()))
                })
                .flatten()
        })
        .filter_map(|v| v.ok_or(()).ok())
        .join("\n");

    if !matches.is_empty() {
        writer.write(path.display().to_string().green().as_bytes()).unwrap();
        writer.write(b"\n").unwrap();
        writer.write(matches.as_bytes()).unwrap();
        writer.write(b"\n").unwrap();
    }

    Ok(())
}

pub fn format_line(line: &str, lineno: usize, range: Range<usize>) -> String {
    let Range { start, end } = range;
    let prefix = &line[..start];
    format!(
        "{0: >6}:{1: <3} {2}{3}{4}",
        lineno.to_string().blue(),
        (prefix.chars().count()).to_string().cyan(),
        prefix,
        &line[start..end].red(),
        &line[end..]
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_line_should_work() {
        let result = format_line("Hello, Tyr~", 1000, 7..10);
        let expected = format!(
            "{0: >6}:{1: <3} Hello, {2}~",
            "1000".blue(),
            "7".cyan(),
            "Tyr".red()
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn default_strategy_should_work() {
        let path = Path::new("src/main.rs");
        let input = b"hello world!\nhey Tyr!";
        let reader = BufReader::new(&input[..]);
        let pattern = Regex::new(r"he\w+").unwrap();
        let mut writer = Vec::new();
        default_strategy(path, reader, &pattern, &mut writer).unwrap();
        let result = String::from_utf8(writer).unwrap();
        let expected = [
            String::from("src/main.rs"),
            format_line("hello world!", 1, 0..5),
            format_line("hey Tyr!\n", 2, 0..3),
        ];

        assert_eq!(result, expected.join("\n"));
    }
}