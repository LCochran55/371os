use std::env;
use std::fs;
use std::fs::File;
use std::io;

const ARGUMENTS: [&str; 12] = [
    "--bytes",
    "--chars",
    "--lines",
    "--max-line-length",
    "--words",
    "--help",
    "--version",
    "-c",
    "-m",
    "-l",
    "-L",
    "-w",
];

const CHAR_ARGUMENTS: [char; 5] = ['c', 'm', 'l', 'L', 'w'];

fn read_file(file: &str) -> std::io::Result<()> {
    let contents = fs::read_to_string(file)?;
    Ok(())
}

fn line_count(file: &str) -> usize {
    let contents: Vec<String> = fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    return contents.len();
}

fn word_count(file: &str) -> usize {
    let mut wc = 0;
    let contents: Vec<String> = fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    for content in contents.iter() {
        let words: Vec<_> = content.split_whitespace().collect::<Vec<_>>();
        wc += words.len();
    }
    wc
}

fn char_count(file: &str) -> usize {
    let contents: Vec<String> = fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let chars: Vec<char> = contents.iter().flat_map(|s| s.chars()).collect();
    return chars.len();
}

fn byte_count(file: &str) -> usize {


    let contents: Vec<String> = fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut count = 0;

    for content in contents.iter() {
        count += content.len();
    }
    count += contents.len();

    return count
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut options: Vec<&str> = Vec::new();
    let mut file: Vec<&str> = Vec::new();

    for i in 1..args.len() {
        match args[i].chars().next() {
            Some('-') => options.push(&args[i]),

            Some(_) => file.push(&args[i]),

            None => println!("lala"),
        }
    }

    let mut base_opts: Vec<String> = options
        .clone()
        .into_iter()
        .filter(|x| ARGUMENTS.contains(x))
        .map(|x| {
            x.replace("--", "");
            x.replace("-", "")
        })
        .collect();
    let mut other_opts: Vec<String> = options
        .clone()
        .into_iter()
        .filter(|x| !ARGUMENTS.contains(x) && !x.contains("--"))
        .flat_map(|x| x.chars())
        .filter(|x| CHAR_ARGUMENTS.contains(x))
        .map(|c| c.to_string())
        .collect();

    base_opts.append(&mut other_opts);

    base_opts.sort();
    base_opts.dedup();

    let mut bytes = String::new();
    let mut chars = String::new();
    let mut lines = String::new();
    let mut words = String::new();
    let mut max_len = String::new();


    if base_opts.len() == 0 {
        println!(
            " {:?} {:?} {:?}",
            line_count(file[0]),
            word_count(file[0]),
            byte_count(file[0])
        );
        return;
    } else {
        for opt in base_opts.iter() {
            match opt.as_str() {
                "c" | "bytes" => let mut bytes = String::from(" ") + &byte_count(file[0]).to_string(),
                "m" | "chars" => let mut chars = String::from(" ") + &char_count(file[0]).to_string(),
                "l" | "lines" => let mut lines = String::from(" ") + &line_count(file[0]).to_string(),
                "L" | "maxlinelength" => println!("max len"),
                "w" | "words" => let mut words = String::from(" ") + &word_count(file[0]).to_string(),
                "help" => println!("help"),
                "version" => println!("ver"),
                _ => println!("lala"),
            }
        }

        println!("{}{}{}{}", lines, words, bytes, chars);
    }
}
