#![allow(dead_code)]
mod memory;
mod loops;
mod matching;
mod decisions;
mod options;
mod functions;
mod traits;
mod borrowing;

use std::env;

const OPT_ALLOCATION: &'static str = "-a";
const OPT_LOOPING: &'static str = "-l";
const OPT_BORROWING: &'static str = "-b";
const OPT_MATCHING: &'static str = "-m";
const OPT_DECISIONS: &'static str = "-d";
const OPT_OPTIONS: &'static str = "-o";
const OPT_FUNCTIONS: &'static str = "-f";
const OPT_TRAITS: &'static str = "-t";

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        show_help();
        return
    }

    let option = args[1].to_owned();
    match_option(option);
}

fn show_help() {
    println!("Please, provide any of these options:");
    println!("");
    write_option_help("Allocation", OPT_ALLOCATION);
    write_option_help("Borrowign", OPT_BORROWING);
    write_option_help("Loops", OPT_LOOPING);
    write_option_help("Matching", OPT_MATCHING);
    write_option_help("Decisions", OPT_DECISIONS);
    write_option_help("Options", OPT_OPTIONS);
    write_option_help("Functions", OPT_FUNCTIONS);
    write_option_help("Traits", OPT_TRAITS);
    println!("");
}

fn write_option_help(literal: &'static str, option: &'static str) {
    println!("{}{}", format!("{: <15}", literal), option);
}

fn match_option(option: String) {
    let _option = &*option;
    match _option {
        OPT_ALLOCATION =>  memory::some_mem_allocation(),
        OPT_LOOPING => loops::some_looping(),
        OPT_BORROWING => borrowing::some_borrowings(),
        OPT_MATCHING => matching::some_mathing_patterns(),
        OPT_DECISIONS => decisions::some_decisions(),
        OPT_OPTIONS => options::find_values(),
        OPT_FUNCTIONS => functions::some_functions(),
        OPT_TRAITS => traits::some_traits(),
        _ => show_help()
    }
}
