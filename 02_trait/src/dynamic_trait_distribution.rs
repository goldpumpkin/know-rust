use std::arch::aarch64::vcvtx_f32_f64;

pub trait Formatter {
    fn format(&self, input: &mut String) -> bool;
}

struct MarkdownFormatter;

impl Formatter for MarkdownFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nformatted with Markdown formatter");
        true
    }
}

struct RustFormatter;

impl Formatter for RustFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nformatted with Rust formatter");
        true
    }
}

pub fn format(input: &mut String, formatters: Vec<&dyn Formatter>) {
    for formatter in formatters {
        formatter.format(input);
    }
}

fn main() {
    let mut text = "Hello Rust".to_string();
    let markdown: &dyn Formatter = &MarkdownFormatter;
    let rust: &dyn Formatter = &RustFormatter;
    let formatters = vec![markdown, rust];
    format(&mut text, formatters);

    println!("format result:{}", text);
}