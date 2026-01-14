use std::env;

fn read_lines(file: &str) -> Vec<String> {
    let mut lines = Vec::new();

    for line in std::fs::read_to_string(file).unwrap().lines() {
        lines.push(line.to_string());
    }
    lines
}

fn words_per_line(line: &String) -> u64 {
    let mut count = 0;
    if line.chars().nth(0).is_none(){
        return 0;
    } else {
        if line.chars().nth(0).is_some() && line.chars().nth(0) != Some(' '){
            count += 1;
        }
        for i in 0..line.chars().count() {
            if line.chars().nth(i) == Some(' ')
                && line.chars().nth(i + 1) != Some(' ')
            {
                count += 1;
            }

        }
    }
    count
}

fn byte_count(line: &String) -> usize {
    let mut count = 0;
    count += line.len();
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let lines = read_lines(file);

    let mut wc = 0;
    let mut bytes = lines.len();

    for line in lines.iter() {
        wc += words_per_line(line);
        bytes += byte_count(line);
    }

    println!("  {}  {} {}", lines.len(), wc, bytes);
}
