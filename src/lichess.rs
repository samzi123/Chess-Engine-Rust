use crate::minimax;

use minimax::calculate_next_move;
use lambda_runtime::Error;
use std::env;
use std::time::Duration;
use pleco::Board;
use serde_json::Value;

// finds all current games of the bot and makes a move in those where it is the bot's turn
pub async fn make_moves_in_all_ongoing_games() -> Result<(), Error> {
    let LICHESS_ACCESS_TOKEN = env::var("RUST_BOT_TOKEN").expect("You need to set a Lichess personal access token.");

    let ongoing_games = get_ongoing_games().await?;
    println!("ongoing_games: {}", ongoing_games);
    let ongoing_games_object: Value = serde_json::from_str(&ongoing_games).unwrap();
    let game_array_json = &ongoing_games_object["nowPlaying"];

    if game_array_json == &Value::Null && game_array_json == &Value::Array(vec![]) {
        println!("No ongoing games");
        return Ok(());
    }

    // loop through all current games and make a move in each
    if let Some(games) = game_array_json.as_array() {
        for game in games {
            let game_id = &game["gameId"].as_str().unwrap();
            let fen = &game["fen"].as_str().unwrap();
            let is_my_turn = &game["isMyTurn"].as_bool().unwrap();

            if *is_my_turn == true {
                let board = Board::from_fen(fen).unwrap();
                let next_move = calculate_next_move(board);
                println!("making move: {}", next_move);

                make_move(&game_id, &next_move).await?;
            }
        }
    }

    Ok(())
}

// calls API to make a move in the given game. Move should be a string in UCI format, e.g. "e2e4"
async fn make_move(game_id: &str, move_to_play: &str) -> Result<String, Error> {
    let LICHESS_ACCESS_TOKEN = env::var("RUST_BOT_TOKEN").expect("You need to set a Lichess personal access token.");
    let URL = "https://lichess.org/api/bot/game/".to_owned() + game_id + "/move/" + move_to_play;

    let client = reqwest::Client::new();
    let response = client
        .post(URL)
        .header("Authorization", "Bearer ".to_owned() + LICHESS_ACCESS_TOKEN.as_str())
        .timeout(Duration::from_secs(5))
        .send()
        .await?
        .text()
        .await?;

    println!("response after making a move: {}", response);

    Ok(response)
}

// fetches all current challenges for the bot, and accepts all that are at least 1 day per turn. It declines the rest
pub async fn respond_to_all_challenges() -> Result<(), Error> {
    let LICHESS_ACCESS_TOKEN = env::var("RUST_BOT_TOKEN").expect("You need to set a Lichess personal access token.");

    let current_challenges = list_current_challenges().await?;
    let challenges_object: Value = serde_json::from_str(&current_challenges).unwrap();
    
    let challenges_received = &challenges_object["in"];
    if challenges_received == &Value::Null && challenges_received == &Value::Array(vec![]) {
        println!("No challenges received");
        return Ok(());
    }

    // loop through challenges_received and accept all that are at least 1 day per turn
    if let Some(challenges) = challenges_received.as_array() {
        for challenge in challenges {
            let challenge_id = &challenge["id"].as_str().unwrap();
            println!("challenge_id: {}", challenge_id);

            let days_per_turn = &challenge["timeControl"]["daysPerTurn"];
            println!("days_per_turn: {}", days_per_turn);
            
            if days_per_turn == &Value::Null {
                // decline challenge as only accepting games with at least 1 day per turn
                println!("declining challenge");
                decline_challenge(challenge_id).await?;
            }
            else {
                // accept challenge
                println!("accepting challenge - need to re-enable");
                accept_challenge(challenge_id).await?;
            }
        }
    }

    Ok(())
}

// fetches all ongoing games for the bot
async fn get_ongoing_games() -> Result<String, Error> {
    let LICHESS_ACCESS_TOKEN = env::var("RUST_BOT_TOKEN").expect("You need to set a Lichess personal access token.");

    let client = reqwest::Client::new();
    let response = client
        .get("https://lichess.org/api/account/playing")
        .header("Authorization", "Bearer ".to_owned() + LICHESS_ACCESS_TOKEN.as_str())
        .timeout(Duration::from_secs(5))
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

// accepts a challenge with the given challenge ID
async fn accept_challenge(challenge_id: &str) -> Result<(), Error> {
    let LICHESS_ACCESS_TOKEN = env::var("RUST_BOT_TOKEN").expect("You need to set a Lichess personal access token.");
    let URL = "https://lichess.org/api/challenge/".to_owned() + challenge_id + "/accept";
    println!("URL: {}", URL);

    let client = reqwest::Client::new();
    let response = client
        .post(URL)
        .header("Authorization", "Bearer ".to_owned() + LICHESS_ACCESS_TOKEN.as_str())
        .timeout(Duration::from_secs(5))
        .send()
        .await?
        .text()
        .await?;

    println!("response: {}", response);

    Ok(())
}

// declines a challenge with the given challenge ID
async fn decline_challenge(challenge_id: &str) -> Result<(), Error> {
    let LICHESS_ACCESS_TOKEN = env::var("RUST_BOT_TOKEN").expect("You need to set a Lichess personal access token.");
    let URL = "https://lichess.org/api/challenge/".to_owned() + challenge_id + "/decline";
    println!("URL: {}", URL);

    let client = reqwest::Client::new();
    let response = client
        .post(URL)
        .header("Authorization", "Bearer ".to_owned() + LICHESS_ACCESS_TOKEN.as_str())
        .body("declineTooFast")
        .timeout(Duration::from_secs(5))
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}

async fn list_current_challenges() -> Result<String, Error> {
    let LICHESS_ACCESS_TOKEN = env::var("RUST_BOT_TOKEN").expect("You need to set a Lichess personal access token.");

    let client = reqwest::Client::new();
    let response = client
        .get("https://lichess.org/api/challenge")
        .header("Authorization", "Bearer ".to_owned() + LICHESS_ACCESS_TOKEN.as_str())
        .timeout(Duration::from_secs(5))
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}