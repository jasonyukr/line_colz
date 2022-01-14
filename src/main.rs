use std::io::{self, BufRead};
use std::env;
use std::path::Path;

/*
ANSI 256color to true-color
===========================
 NAME       FG/BG     RGB
---------------------------
white      37m/47m   c5c8c6
red        31m/41m   cc6666
green      32m/42m   b5bd68
yellow     33m/43m   f0c674
blue       34m/44m   81a2be
magenta    35m/45m   b294bb
cyan       36m/46m   8abe87
gray       90m/100m  666666
Br white   97m/107m  eaeaea
Br red     91m/101m  d54e53
Br green   92m/102m  b9ca4a
Br yellow  93m/103m  e7c547
Br blue    94m/104m  7aa6da
Br magenta 95m/105m  c397d8
Br cyan    96m/106m  70c0b1
===========================
*/

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
    // Table for true-color gradation
    // (step, start.r, start.g, start.b, end.r, end.g, end.b)
    let gradation_table =
        [(50, 0xc5, 0xc8, 0xc6, 0xb0, 0xb0, 0xb0),
        (50, 0xcc, 0x66, 0x66, 0xb0, 0xb0, 0xb0),
        (50, 0xb5, 0xbd, 0x68, 0xb0, 0xb0, 0xb0),
        (50, 0xf0, 0xc6, 0x74, 0xb0, 0xb0, 0xb0),
        (50, 0x81, 0xa2, 0xbe, 0xb0, 0xb0, 0xb0),
        (50, 0xb2, 0x94, 0xbb, 0xb0, 0xb0, 0xb0),
        (50, 0x8a, 0xbe, 0x87, 0xb0, 0xb0, 0xb0),
        (50, 0x66, 0x66, 0x66, 0xb0, 0xb0, 0xb0),
        (50, 0xea, 0xea, 0xea, 0xb0, 0xb0, 0xb0),
        (50, 0xd5, 0x4e, 0x53, 0xb0, 0xb0, 0xb0),
        (50, 0xb9, 0xca, 0x4a, 0xb0, 0xb0, 0xb0),
        (50, 0xe7, 0xc5, 0x47, 0xb0, 0xb0, 0xb0),
        (50, 0x7a, 0xa6, 0xda, 0xb0, 0xb0, 0xb0),
        (50, 0xc3, 0x97, 0xd8, 0xb0, 0xb0, 0xb0),
        (50, 0x70, 0xc0, 0xb1, 0xb0, 0xb0, 0xb0),
        (100, 0xc5, 0xc8, 0xc6, 0xb0, 0xb0, 0xb0),
        (100, 0xcc, 0x66, 0x66, 0xb0, 0xb0, 0xb0),
        (100, 0xb5, 0xbd, 0x68, 0xb0, 0xb0, 0xb0),
        (100, 0xf0, 0xc6, 0x74, 0xb0, 0xb0, 0xb0),
        (100, 0x81, 0xa2, 0xbe, 0xb0, 0xb0, 0xb0),
        (100, 0xb2, 0x94, 0xbb, 0xb0, 0xb0, 0xb0),
        (100, 0x8a, 0xbe, 0x87, 0xb0, 0xb0, 0xb0),
        (100, 0x66, 0x66, 0x66, 0xb0, 0xb0, 0xb0),
        (100, 0xea, 0xea, 0xea, 0xb0, 0xb0, 0xb0),
        (100, 0xd5, 0x4e, 0x53, 0xb0, 0xb0, 0xb0),
        (100, 0xb9, 0xca, 0x4a, 0xb0, 0xb0, 0xb0),
        (100, 0xe7, 0xc5, 0x47, 0xb0, 0xb0, 0xb0),
        (100, 0x7a, 0xa6, 0xda, 0xb0, 0xb0, 0xb0),
        (100, 0xc3, 0x97, 0xd8, 0xb0, 0xb0, 0xb0),
        (100, 0x70, 0xc0, 0xb1, 0xb0, 0xb0, 0xb0)];

    let mut gradation_idx = 0;

    let mut check_file = false;
    let mut reverse = false;
    let mut split = false;

    // parse argument
    let mut idx_mode = false;
    for arg in env::args() {
        if idx_mode {
            gradation_idx = arg.parse::<usize>().unwrap();
            if gradation_idx >= gradation_table.len() {
                gradation_idx = 0;
            }
            idx_mode = false;
            continue;
        }
        if arg == "-f" || arg == "--f" {
            check_file = true;
        } else if arg == "-r" || arg == "--r" {
            reverse = true;
        } else if arg == "-g" || arg == "--g" {
            idx_mode = true
        } else if arg == "-s" || arg == "--s" {
            split = true;
        }
    }

    let stdin = io::stdin();
    let mut v = vec![];
    for ln in stdin.lock().lines() {
        let line = ln.unwrap();
        if split {
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
        } else {
            let data = Data{d1: "".to_string(), d2: line};
            v.push(data);
        }
    }

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
                if split {
                    println!("\x1b[90m{}\x1b[38;5;{}m{}\x1b[0m", data.d1, 1, data.d2);
                } else {
                    println!("\x1b[38;5;{}m{}\x1b[0m", 1, data.d2);
                }
                continue;
            }
        }

        let mut fore = RGB {
            r: end.r,
            g: end.g,
            b: end.b
        };

        let rgb;
        if reverse {
            rgb = gradation.get(idx);
        } else {
            rgb = gradation.get((line_count - idx as i32) as usize);
        }
        if !rgb.is_none() {
            fore.r = rgb.unwrap().r;
            fore.g = rgb.unwrap().g;
            fore.b = rgb.unwrap().b;
        }

        if split {
            println!("\x1b[90m{}\x1b[38;2;{};{};{}m{}\x1b[0m", data.d1, fore.r, fore.g, fore.b, data.d2);
        } else {
            println!("\x1b[38;2;{};{};{}m{}\x1b[0m", fore.r, fore.g, fore.b, data.d2);
        }
    }
}
