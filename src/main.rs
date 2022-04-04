use std::io;
use std::io::Write;

pub fn prompt_input() -> i32  {
    let input = String::new();

    print!("Input: ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(input)
        .unwrap();

    match input.parse::<i32>() {
        Ok(result) => return result,
        Err(e) => eprintln!("Failed to parse input!\nError:\n{}", e)
    }

    return -1;
}

pub fn handle_input() {
    let mut input: i32 = prompt_input();

}

pub fn display_menu() {
    println!("❱ Ginfin ❰\n");
    println!("Options:\n1) New Entry\n2) View Entries\n3) Exit\n");

}

fn main() {

    //print menu or something
    //Add 'entries'
    //fill up entries with values
    //write entries to file
    //load entries from file
    //display entries
    //calulate graph for
    //  week
    //  month
    //  year
    //display graph

    display_menu();
    handle_input();

}