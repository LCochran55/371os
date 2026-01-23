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

const HELP: &str = "Usage: binkle_wc [OPTION]... [FILE]...
Print newline, word, and byte counts for each FILE, and a total line if
more than one FILE is specified.  A word is a non-zero-length sequence of
characters delimited by white space.\n

With no FILE, or when FILE is -, read standard input.\n

The options below may be used to select which counts are printed, always in
the following order: newline, word, character, byte, maximum line length.\n
  -c, --bytes            print the byte counts\n
  -m, --chars            print the character counts\n
  -l, --lines            print the newline counts\n
  -L, --max-line-length  print the maximum display width\n
  -w, --words            print the word counts\n
      --help     display this help and exit\n
      --version  output version information and exit\n

Binkle cannot help you if you have any problems";

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
    let white_space = contents.len();
    let chars: Vec<char> = contents.iter().flat_map(|c| c.chars()).collect();
    return chars.len() + contents.len();
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

    return count;
}

fn max_line_count(file: &str) -> usize {
    let contents: Vec<String> = fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut largest = 0;

    for content in contents.iter() {
        let line_len = content.len();
        if line_len > largest {
            largest = line_len;
        }
    }
    return largest;
}

fn blank_input(file: &str) {
    let mut bytes = byte_count(file);
    let mut lines = line_count(file);
    let mut words = word_count(file);

    println!("{} {} {}", lines, words, bytes);
}

fn no_file() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut options: Vec<&str> = Vec::new();
    let mut file: Vec<String> = Vec::new();

    for i in 1..args.len() {
        match args[i].chars().next() {
            Some('-') => options.push(&args[i]),

            Some(_) => file.push(args[i].clone()),

            _ => (),

        }
    }

    if file.len() == 0 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        file.push(input.trim().to_string());
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
            " {:?} {:?} {:?} {:?}",
            line_count(&file[0]),
            word_count(&file[0]),
            byte_count(&file[0]),
            file[0]
        );
        return;
    } else {
        for opt in base_opts.iter() {
            match opt.as_str() {
                "c" | "bytes" => bytes = String::from(" ") + &byte_count(&file[0]).to_string(),
                "m" | "chars" => chars = String::from(" ") + &char_count(&file[0]).to_string(),
                "l" | "lines" => lines = String::from(" ") + &line_count(&file[0]).to_string(),
                "L" | "maxlinelength" => {
                    max_len = String::from(" ") + &max_line_count(&file[0]).to_string()
                }
                "w" | "words" => words = String::from(" ") + &word_count(&file[0]).to_string(),
                "help" => println!("{}",HELP),
                "version" => println!("Binkle_wc Version 1.0!"),
                _ => {
                    blank_input(&file[0]);
                    return;
                }
            }
        }

        println!("{}{}{}{}{} {:?}", lines, words, bytes, chars, max_len,file[0]);
    }
}

