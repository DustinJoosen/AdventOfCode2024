use std::fs;
use std::time::Instant;

mod days;

fn main() {
    let selected_day: i32 = 7;

    println!("================================================\n\
    Welcome to Advent of Code 2024!\n\
    We will now be running day \x1b[93m{}\x1b[0m.\n",
             selected_day);

    // Get file contents.
    let lines = get_file_contents(selected_day);

    // Start timer.
    let start_time = Instant::now();

    // Run code.
    println!("\nStart!");
    let result: i32 = match selected_day {
        1 => days::day01::run(lines),
        2 => days::day02::run(lines),
        3 => days::day03::run(lines),
        4 => days::day04::run(lines),
        5 => days::day05::run(lines),
        6 => days::day06::run(lines),
        7 => days::day07::run(lines),
        _ => {
            println!("This day is not yet implemented");
            -1
        }
    };

    // Stop timer.
    let end_time = start_time.elapsed();

    println!("Done!\nResult: {result}\n");
    println!("Code finished in {} ms", end_time.as_millis());
}

fn get_file_contents(day: i32) -> String {
    println!("Getting file contents...");

    let formatted_day: String = (100 + day)
        .to_string()
        .chars()
        .skip(1)
        .collect();

    let file_path = format!("src/inputs/day{}.txt", formatted_day);
    let lines = fs::read_to_string(file_path)
        .unwrap_or_default();

    println!("File contents found!");

    lines.replace("\r", "")
}
