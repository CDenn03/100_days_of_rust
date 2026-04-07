use colored::Colorize;
use std::io;


enum Verdict {
    Win,
    Lose,
    Draw,
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct MatchResult {
    player_score: i32,
    opponent_score: i32,
    rounds: i32,
}

fn generate_opponent_move() -> i32 {
    rand::random_range(1..=3)
}

fn get_round_result(player_move: &i32) -> Verdict {
    let opponent_move = generate_opponent_move();


    let player_move_enum = match player_move {
        1 => Move::Rock,
        2 => Move::Paper,
        _ => Move::Scissors,
    };

    let opponent_move_enum = match opponent_move {
        1 => Move::Rock,
        2 => Move::Paper,
        _ => Move::Scissors,
    };

    let space_between = (opponent_move - player_move + 3) % 3;

    let verdict = match space_between {
        0 => Verdict::Draw,
        1 => Verdict::Lose,
        _ => Verdict::Win,
    };

    print_round_result(player_move_enum, opponent_move_enum, &verdict);
    verdict
}

fn update_match_result(round_result: Verdict, match_result: &mut MatchResult) {
    match round_result {
        Verdict::Win => match_result.player_score += 1,
        Verdict::Lose => match_result.opponent_score += 1,
        Verdict::Draw => {}
    }

    match_result.rounds += 1;
}

fn get_player_input() -> i32 {
    loop {
        println!("{}", "Pick a number to play(then hit Enter)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) if (1..=3).contains(&num) => return num,
            _ => println!("{}", "Please pick a number between 1 and 3".red()),
        }
    }
}

fn print_round_result(player_move: Move, opponent_move: Move, verdict: &Verdict) {
    println!(
        "{}",
        format!(
            "You played {:?}, opponent played {:?}",
            player_move, opponent_move
        )
            .blue()
    );

    match verdict {
        Verdict::Win => println!("{}", "You win!".green()),
        Verdict::Lose => println!("{}", "Opponent wins!".red()),
        Verdict::Draw => println!("{}", "It's a draw!".yellow()),
    }
}

fn print_match_result(match_result: &MatchResult) {
    println!(" ");

    let summary = format!(
        "Final Score:\nRounds: {}\nYou: {}\nOpponent: {}",
        match_result.rounds,
        match_result.player_score,
        match_result.opponent_score
    );

    if match_result.player_score > match_result.opponent_score {
        println!("{}", summary.green().bold());
        println!("{}", "\nYou won the match!".green().bold());
    } else if match_result.player_score < match_result.opponent_score {
        println!("{}", summary.red().bold());
        println!("{}", "\nYou lost the match!".red().bold());
    } else {
        println!("{}", summary.yellow().bold());
        println!("{}", "\nIt's a draw!".yellow().bold());
    }
}

fn ask_play_again() -> bool {
    loop {
        println!("{}", "Do you want to play again? (y/n)".yellow());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("{}", "\nPlease enter 'y' or 'n'".red()),
        }
    }
}

fn main() {
    loop {

        println!(" ");
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

        let mut match_result = MatchResult {
            player_score: 0,
            opponent_score: 0,
            rounds: 0,
        };

        for _ in 1..=rounds {
            println!("\n");

            println!(
                "{}",
                format!(". Round {}/{} .", match_result.rounds + 1,&rounds)
                    .on_bright_cyan()
                    .bold()
            );

            let player_pick = get_player_input();

            let round_result = get_round_result(&player_pick);
            update_match_result(round_result, &mut match_result);
        }

        print_match_result(&match_result);

        if !ask_play_again() {
            println!("{}", "Thanks for playing!".green().bold());
            break;
        }
    }
}