use std::io::{self, BufRead};
use std::env;
use std::path::Path;

struct Data {
    d1: String,
    d2: String 
}

fn min(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    if a == None && b == None {
        return None;
    }
    if a == None {
        return b;
    }
    if b == None {
        return a;
    }
    return std::cmp::min(a, b);
}

fn main() {
    let t1 = [(10 * 1, 255),
            (10 * 2, 254),
            (10 * 3, 253),
            (10 * 4, 252),
            (10 * 5, 251),
            (10 * 6, 250),
            (9999999, 249)];
    let t2 = [(10 * 1, 208),
            (10 * 2, 172),
            (10 * 3, 136),
            (10 * 4, 100),
            (10 * 5, 64),
            (10 * 6, 64),
            (9999999, 250)];
    let t3 = [(10 * 1, 211),
            (10 * 2, 175),
            (10 * 3, 139),
            (10 * 4, 103),
            (10 * 5, 67),
            (10 * 6, 31),
            (9999999, 250)];
    let t4 = [(10 * 1, 207),
            (10 * 2, 206),
            (10 * 3, 205),
            (10 * 4, 204),
            (10 * 5, 203),
            (10 * 6, 202),
            (9999999, 250)];
    let t5 = [(10 * 1, 213),
            (10 * 2, 177),
            (10 * 3, 141),
            (10 * 4, 105),
            (10 * 5, 69),
            (10 * 6, 33),
            (9999999, 250)];
    let t6 = [(10 * 1, 3),
            (10 * 2, 5),
            (10 * 3, 4),
            (10 * 4, 6),
            (10 * 5, 2),
            (10 * 6, 2),
            (9999999, 250)];
    let mut table = t1;
    let mut check_file = false;
    let mut reverse = false;

    // parse argument
    for arg in env::args() {
        if arg == "-t1" {
            table = t1;
        } else if arg == "-t2" {
            table = t2;
        } else if arg == "-t3" {
            table = t3;
        } else if arg == "-t4" {
            table = t4;
        } else if arg == "-t5" {
            table = t5;
        } else if arg == "-t6" {
            table = t6;
        } else if arg == "-f" {
            check_file = true;
        } else if arg == "-r" {
            reverse = true;
        }
    }

    let stdin = io::stdin();
    let mut v = vec![];
    for ln in stdin.lock().lines() {
        let line = ln.unwrap();
        let line_trim = line.trim();
        let idx = min(line_trim.find(" "), line_trim.find("\t"));

        let e1;
        let e2;
        if idx == None {
            e1 = "";
            e2 = line_trim;
        } else {
            e1 = &line_trim[..idx.unwrap()];
            e2 = &line_trim[idx.unwrap()..];
        }
        let data = Data{d1: e1.to_string(), d2: e2.to_string()};
        v.push(data);
    }

    let line_count = v.len() as i32;
    for (idx, data) in v.iter().enumerate() {
        let ln = (idx + 1) as i32;
        if check_file {
            if !Path::new(data.d2.trim()).exists() {
                println!("\x1b[90m{}\x1b[38;5;{}m{}\x1b[0m", data.d1, 1, data.d2);
                continue;
            }
        }
        if reverse {
            for t in table {
                if ln <= t.0 {
                    println!("\x1b[90m{}\x1b[38;5;{}m{}\x1b[0m", data.d1, t.1, data.d2);
                    break;
                }
            }
        } else {
            for t in table {
                if ln > line_count - t.0 {
                    println!("\x1b[90m{}\x1b[38;5;{}m{}\x1b[0m", data.d1, t.1, data.d2);
                    break;
                }
            }
        }
    }
}
