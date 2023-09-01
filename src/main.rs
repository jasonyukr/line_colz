use std::io::{self, BufRead};
use std::env;
use std::path::Path;

/*
ANSI 256color to true-color
===========================
 NAME       FG/BG     Rgb
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

#[derive(Debug, Copy, Clone)]
struct Rgb {
    r: u32,
    g: u32,
    b: u32
}

fn get_grad(start: &Rgb, end: &Rgb, steps: u32) -> Vec<Rgb> {
    let mut alpha = 0.0;
    let mut grad = Vec::new();

    for _ in 0..steps {
        alpha = alpha + (1.0 / steps as f32);

        let red = end.r as f32 * alpha + (1.0 - alpha) * start.r as f32;
        let green = end.g as f32 * alpha + (1.0 - alpha) * start.g as f32;
        let blue = end.b as f32 * alpha + (1.0 - alpha) * start.b as f32;

        let rgb = Rgb {
            r: red as u32,
            g: green as u32,
            b: blue as u32
        };
        grad.push(rgb);
    }
    grad
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
    let grad_table =
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

    let mut grad_idx = 0;

    let mut check_exist = false;
    let mut reverse = false;
    let mut split = false;

    // parse argument
    let mut idx_mode = false;
    for arg in env::args() {
        if idx_mode {
            if let Ok(i) = arg.parse::<usize>() {
                grad_idx = i;
                if grad_idx >= grad_table.len() {
                    grad_idx = 0;
                }
            }
            idx_mode = false;
            continue;
        }
        if arg == "-f" || arg == "--f" {
            check_exist = true;
        } else if arg == "-d" || arg == "--d" {
            check_exist = true;
        } else if arg == "-e" || arg == "--e" {
            check_exist = true;
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
        if let Ok(line) = ln {
            if split {
                let line_trim = line.trim();
                if let Some(i) = min(line_trim.find(' '), line_trim.find('\t')) {
                    v.push(((&line_trim[..i]).to_string(), (&line_trim[i..]).to_string()));
                } else {
                    v.push((String::from(""), line_trim.to_string()));
                }
            } else {
                v.push((String::from(""), line));
            }
        }
    }

    let fore_start_color = Rgb {
        r: grad_table[grad_idx].1,
        g: grad_table[grad_idx].2,
        b: grad_table[grad_idx].3
    };
    let fore_end_color = Rgb {
        r: grad_table[grad_idx].4,
        g: grad_table[grad_idx].5,
        b: grad_table[grad_idx].6
    };

    let grad = get_grad(&fore_start_color, &fore_end_color, grad_table[grad_idx].0);
    let line_count = v.len() as i32;
    for (idx, data) in v.iter().enumerate() {
        if check_exist {
            let path = shellexpand::tilde(data.1.trim()).into_owned();
            if !Path::new(&path).exists() {
                if split {
                    println!("\x1b[90m{}\x1b[38;5;{}m{}\x1b[0m", data.0, 1, data.1);
                } else {
                    println!("\x1b[38;5;{}m{}\x1b[0m", 1, data.1);
                }
                continue;
            }
        }

        // get current gradation color from current index
        let grad_color;
        if reverse {
            grad_color = grad.get(idx);
        } else {
            grad_color = grad.get((line_count - idx as i32) as usize);
        }

        let mut fore_color = &fore_end_color;
        if let Some(c) = grad_color {
            fore_color = c;
        }

        if split {
            println!("\x1b[90m{}\x1b[38;2;{};{};{}m{}\x1b[0m",
                     data.0,
                     fore_color.r, fore_color.g, fore_color.b,
                     data.1);
        } else {
            println!("\x1b[38;2;{};{};{}m{}\x1b[0m",
                     fore_color.r, fore_color.g, fore_color.b,
                     data.1);
        }
    }
}

