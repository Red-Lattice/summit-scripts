use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use std::time::SystemTime;

fn main() {
    println!();
    let now = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let file_name = args.last().unwrap();
    for arg in &args {
        if arg.contains(".exe") || arg == file_name {continue;}
        read_command(&arg, file_name);
    }
    println!("Executed in: {:?}", now.elapsed().unwrap());
    //let start = std::env::args().nth(1).expect("no start given");
    //let end = std::env::args().nth(2).expect("no amount given");
    //take_input();
}
/*fn take_input() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();
    let mut split = trimmed.split_whitespace();
    
    read_command(&split.next().unwrap(), split);
}*/

fn read_command(command: &str, file: &str) {
    match command {
        "search" => match read_file(file) {
                Some(str) => search_for_flag(str),
                None => println!("File failed to read"),
            },
        _ => println!("Command not recognized: {command}")
    }
}

fn open_file(file_to_open: &str) -> File {OpenOptions::new().read(true).open(file_to_open).unwrap()}

fn read_file(file_to_search: &str) -> Option<String> {
    let file = open_file(file_to_search);
    let mut output_string: String = "".to_string();
    return match BufReader::new(file).read_to_string(&mut output_string) {
        Ok(_) => Some(output_string),
        Err(_) => None,
    }
}

fn search_for_flag(output_string: String) {
    let first_index = match output_string.find("flag{") {
        Some(i) => i,
        None => return println!("No flag found"),
    };

    //let second_index = output_string.find("}").unwrap();
    let second_index: usize = match output_string.match_indices("}").find_map(|(i, _)| (i >= first_index).then(|| i)) {
        Some(i) => i,
        None => return println!("Potential flag at character: {}", first_index + 1)
    };

    println!("{}", &output_string[(first_index)..(second_index+1)])
}