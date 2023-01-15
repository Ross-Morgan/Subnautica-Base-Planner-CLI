use std::fmt::Display;
use std::io::{self, BufRead, Write};
use std::str::{self, FromStr};

use colored::Colorize;

use crate::subnautica::prelude::*;

pub fn run_shell() {
    let mut base = Base::new();

    loop {
        let input = get_input("sub>".cyan());
        let command = input
            .split(" ")
            .map(|s| s.as_bytes())
            .collect::<Vec<_>>();

        if parse_command(command.as_slice(), &mut base).is_some() {
            break
        };
    }
}

fn parse_command(command: &[&[u8]], base: &mut Base) -> Option<()> {
    match command {
        &[] => println!("No input supplied"),

        &[b"help", ref _a @ ..] => {
            print_help();
        },

        &[b"exit", ref _a @ ..] => {
            return Some(())
        },

        &[b"add", item, ref b_quantity @ ..]  => {
            let item_name = std::str::from_utf8(item).unwrap();

            let quantity = match b_quantity {
                &[] => {
                    0 as usize
                }
                _ => {
                    let a = args_to_str_vec(b_quantity);

                    a.get(0).unwrap().parse::<usize>().unwrap_or(0)
                }
            };

            let item = match Item::from_str(item_name) {
                Ok(i) => i,
                Err(e) => {
                    println!("Invalid item name specified: {:?}", item_name.red());
                    return None
                }
            };


        }

        &[b"remove", ref b_args @ ..] => {
            let args = args_to_str_vec(b_args);
            println!("{args:?}");
        },
        &[b"clear", ref b_args @ ..] => {
            let args = args_to_str_vec(b_args);
            println!("{args:?}");
        },
        &[b"get", b"depth"] => {
            println!("Current depth: {} metres", base.get_depth().to_string().blue())
        }
        &[b"set", b"depth", b_new_depth] => {
            if b_new_depth.is_empty() {
                println!("New depth not specified");
                return None
            }

            let new_depth = args_to_str_vec(&[b_new_depth]).get(0).unwrap().parse::<i64>();

            match new_depth {
                Ok(depth) => base.set_depth(depth),
                Err(_) => println!("Invalid base depth provided"),
            }
        }

        i => println!("Other input supplied: {:?}", args_to_str_vec(i)),
    };

    None
}

fn get_input<T: Display>(prompt: T) -> String {
    print!("{prompt}");

    let _ = io::stdout().flush().unwrap_or_default();

    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap_or(Ok(Default::default()))
        .unwrap_or(Default::default())
}

fn args_to_str_vec<'a>(b: &[&'a [u8]]) -> Vec<&'a str> {
    b.into_iter()
        .map(|&b| str::from_utf8(b).expect("Failed to convert byte slice to string"))
        .collect()
}

fn print_help() {
    println!("help -");
    println!("\tview this help screen");
    println!();
    println!("add [piece] [quantity=1] -");
    println!("\tadds the specified number of pieces to the base");
    println!("\tdefault number of pieces is 1");
    println!();
    println!("remove [piece] [quantity=1] -");
    println!("\tremoves the specified number of pieces from the base");
    println!("\tdefault number of pieces is 1")
}