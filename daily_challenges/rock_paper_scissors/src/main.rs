use colored::Colorize;
use std::io;

enum Verdict {
    Win,
    Lose,
    Draw
}

struct RoundResult{
    verdict: Verdict,
}
// struct MatchResult{
//
// }
fn main() {
    println!(
        "{}",
        "                   Rock Paper Scissors Game                  "
            .on_blue()
            .bold()
    );
    println!("{}", "Here are the rules: ".blue());
    println!("{}", "Use the numbers to play ... e.g. 1 for Rock");
    println!("{}", "1: Rock ");
    println!("{}", "2: Paper ");
    println!("{}", "3: Scissors ");

    let rounds: i32 = 3;

    for _ in 1..=rounds {
        let pick: i32 = loop {
            println!("{}", "Pick a number to play");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse() {
                Ok(num) => {
                    if num < 1 || num > 3{
                        println!(
                            "{}",
                            "Please pick a number between 1 and 3".red(),
                        );
                        continue;
                    }
                    break num;
                }
                Err(_) => {
                    println!(
                        "{}",
                        "Please pick a number between 1 and 3".red(),
                    );
                    continue;
                }
            };
        };

        getRoundResult(&pick);
    }
}

fn getRoundResult(pick: &i32) {}
fn getMatchResult() {}
