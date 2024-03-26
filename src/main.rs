use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use std::time::SystemTime;

fn main() {
    println!();
    let now = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let file_name = args.last().unwrap();
    let mut file_name_searched: bool = false;
    let mut search_space: String = "".to_string();
    for arg in &args {
        if arg.contains(".exe") || arg == file_name {continue;}
        if !file_name_searched {search_space = read_command_with_file(&arg, &file_name).unwrap(); file_name_searched = true;}
        else {search_space = read_command(&arg, &search_space).unwrap();}
    }
    println!("{search_space}");
    println!("Executed in: {:?}", now.elapsed().unwrap());
}

fn read_command_with_file(command: &str, file: &str) -> Option<String> {
    match command.to_lowercase().as_str() {
        "search" => match read_file(file) {
                Some(str) => search_for_flag(str),
                None => {println!("File failed to read"); None},
            },
        "caedec" => match read_file(file) {
            Some(str) => {decrypt_caesar(&str)},
            None => {println!("File failed to read"); None},
        },
        _ => {println!("Command not recognized: {command}"); None}
    }
}

fn read_command(command: &str, search_space: &str) -> Option<String> {
    match command.to_lowercase().as_str() {
        "search" => search_for_flag(search_space.to_string()),
        _ => {println!("Command not recognized: {command}"); None}
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

pub const CHAR: [char;27] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',' '];

fn decrypt_caesar(input: &str) -> Option<String> {// {
    let mut out: String = "".to_owned();
    for i in 0..26 {
        let mut pos = 0;
        // Good fucking luck
        out.push_str((i.to_string() + if i < 10 {" "} else {""} + " SHIFT: " + (&(input.chars().map(|c| {pos = CHAR.iter().position(|e| e == &(format!("{}",c.to_uppercase()).chars().next().unwrap())).unwrap_or(100 - i) + i; if pos != 100 {pos = pos % 26;}if pos == 100 {c} else {*CHAR.get(pos).unwrap()}}).collect::<String>() + "\n").as_str()).to_owned()).as_str());
    }
    Some(out)
}

fn search_for_flag(output_string: String) -> Option<String> {
    let lower_string = output_string.to_lowercase();
    let first_index = match lower_string.find("flag{") {
        Some(i) => i,
        None => {usize::MAX},
    };
    if first_index == usize::MAX {return Some("No flag found".to_string())}

    let second_index: usize = match output_string.match_indices("}").find_map(|(i, _)| (i >= first_index).then(|| i)) {
        Some(i) => i,
        None => {usize::MAX},
    };
    if second_index == usize::MAX {"Potential flag at character: ".to_owned().push_str((first_index + 1).to_string().as_str())}

    Some(output_string[(first_index)..(second_index+1)].to_string())
}