use std::io::{self, BufRead};
use std::env;
use std::path::Path;

struct Data {
    d1: String,
    d2: String 
}

struct RGB {
    r: u32,
    g: u32,
    b: u32
}

fn generate_gradation(start: &RGB, end: &RGB, steps: u32) -> Vec<RGB> {
    // The number of colors to compute
    let len = steps;

    // Alpha blending amount
    let mut alpha = 0.0;

    let mut gradation: Vec<RGB> = Vec::new();

    for _i in 0..len {
        let red: f32;
        let green: f32;
        let blue: f32;
        alpha = alpha + (1.0 / len as f32);

        red = end.r as f32 * alpha + (1.0 - alpha) * start.r as f32;
        green = end.g as f32 * alpha + (1.0 - alpha) * start.g as f32;
        blue = end.b as f32 * alpha + (1.0 - alpha) * start.b as f32;

        let rgb = RGB {
            r: red as u32,
            g: green as u32,
            b: blue as u32
        };
        gradation.push(rgb)
    }
    return gradation;
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

    // Table for 256 color
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
    let t7 = [(2 * 1, 255),
            (2 * 2, 254),
            (2 * 3, 253),
            (2 * 4, 252),
            (2 * 5, 251),
            (2 * 6, 250),
            (9999999, 249)];
    let mut table = t1;

    // Table for true-color gradation
    let gradation_table = [(1, 255, 255, 255, 255, 255, 255),
                    (100, 0xff, 0xff, 0xff, 0x99, 0x99, 0x99),
                    (100, 0x65, 0xa8, 0xef, 0x99, 0x99, 0x99),
                    (100, 0xc3, 0xa0, 0x21, 0x99, 0x99, 0x99),
                    (100, 0x7e, 0xa4, 0x5b, 0x99, 0x99, 0x99)];
    let mut gradation_idx = 0;

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
        } else if arg == "-t7" {
            table = t7;
        } else if arg == "-g1" {
            gradation_idx = 1;
        } else if arg == "-g2" {
            gradation_idx = 2;
        } else if arg == "-g3" {
            gradation_idx = 3;
        } else if arg == "-g4" {
            gradation_idx = 4;
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

    if gradation_idx > 0 {
        let start = RGB {
            r: gradation_table[gradation_idx].1,
            g: gradation_table[gradation_idx].2,
            b: gradation_table[gradation_idx].3
        };
        let end = RGB {
            r: gradation_table[gradation_idx].4,
            g: gradation_table[gradation_idx].5,
            b: gradation_table[gradation_idx].6
        };

        let gradation = generate_gradation(&start, &end, gradation_table[gradation_idx].0);
        let line_count = v.len() as i32;
        for (idx, data) in v.iter().enumerate() {
            if check_file {
                if !Path::new(data.d2.trim()).exists() {
                    println!("\x1b[90m{}\x1b[38;5;{}m{}\x1b[0m", data.d1, 1, data.d2);
                    continue;
                }
            }
            let rgb;
            if reverse {
                rgb = gradation.get((line_count - idx as i32) as usize);
            } else {
                rgb = gradation.get(idx);
            }
            let red: u32;
            let green: u32;
            let blue: u32;
            if rgb.is_none() {
                red = end.r;
                green = end.g;
                blue = end.b;
            } else {
                red = rgb.unwrap().r;
                green = rgb.unwrap().g;
                blue = rgb.unwrap().b;
            }
            println!("\x1b[90m{}\x1b[38;2;{};{};{}m{}\x1b[0m", data.d1, red, green, blue, data.d2);
        }
    } else {
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
}
