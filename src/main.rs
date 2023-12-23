use std::env::args as prog_args;
use std::fs::File;
use std::io::stdin;
use std::io::Read;
use std::io::Write;

fn main() {
    let mut args = prog_args();
    let _prog_name = args.next().unwrap();
    if args.next().is_none() {
        let mut input_stream = stdin();
        let mut input = String::new();
        input_stream.read_to_string(&mut input).unwrap();
        let fixed_widows = fix_widows(&input);
        println!("{fixed_widows}");
    } else {
        let tex_path = args
            .next()
            .expect("You need to provide a filepath to .tex file");
        if !tex_path.ends_with(".tex") {
            panic!("You need to provide a filepath to .tex file");
        }

        let mut tex_file = File::open(tex_path).unwrap();
        let mut text = String::new();
        let _bytes_read = tex_file.read_to_string(&mut text).unwrap();

        let fixed_widows = fix_widows(&text);
        println!("{fixed_widows}");
        if let Some(output_path) = args.next() {
            let mut out_file = File::create(output_path).unwrap();
            out_file
                .write_all(fixed_widows.as_bytes())
                .expect("File to be written");
        }
    }
}

const DEWIDOWIZE_KEYWORDS: [&str; 5] = [
    r"\section",
    r"\subsection",
    r"\subsubsection",
    r"\caption",
    r"\footnote",
];

fn fix_widows(t: &str) -> String {
    let lines = t.split_inclusive('\n');
    let mut words_with_delims = Vec::new();
    lines.for_each(|line| words_with_delims.extend(line.split_inclusive(' ')));
    let mut fixed = String::new();
    let mut skipping = false;
    let mut skipping_envs = false;
    for word in &words_with_delims {
        if word.contains(r"\begin{verbatim}") {
            skipping_envs = true;
        }
        if word.contains('{') {
            let tex_command = word.split_once('{').unwrap().0;
            if !DEWIDOWIZE_KEYWORDS.contains(&tex_command) {
                skipping = true;
            }
        }
        if word.contains('}') {
            skipping = false;
        }
        if word.contains(r"\end{verbatim}") {
            skipping_envs = false;
        }
        if word.len() == 2 && !skipping && !word.contains('}') && !skipping_envs {
            let widow = word.chars().next().unwrap();
            fixed.push(widow);
            fixed.push('~');
        } else {
            fixed.push_str(word);
        }
    }
    fixed
}
