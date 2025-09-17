// vault_rpg/src/puzzles.rs

use std::io::{self, Write};

use crate::totp::{TotpConfig, get_totp_code};

pub fn run_puzzle_game() -> bool {
    println!("📜 Before you stands a stone tablet engraved with two puzzle types:");
    println!("1. Enter TOTP code");
    println!("2. Custom fixed question");
    print!("Please select puzzle type (1/2): ");
    io::stdout().flush().unwrap();
    let mut puzzle_choice = String::new();
    io::stdin().read_line(&mut puzzle_choice).unwrap();
    let puzzle_choice = puzzle_choice.trim();

    match puzzle_choice {
        "1" => {
            println!("\n--- Puzzle One: Enter TOTP Code ---");
            
            // 从环境变量获取 TOTP 配置
            let secret = match std::env::var("VAULT_TOTP_SECRET") {
                Ok(secret) => secret,
                Err(_) => {
                    println!("❌ TOTP secret not configured. Please set VAULT_TOTP_SECRET environment variable.");
                    return false;
                }
            };
            
            let account = std::env::var("VAULT_TOTP_ACCOUNT").unwrap_or_else(|_| "default@example.com".to_string());
            let issuer = std::env::var("VAULT_TOTP_ISSUER").ok();
            
            println!("Please enter the current TOTP verification code for {}:\n", account);
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let answer = input.trim();
            
            // 使用环境变量配置的 TOTP
            let config = TotpConfig {
                algorithm: totp_rs::Algorithm::SHA1,
                digits: 6,
                skew: 1,
                step: 30,
                secret,
                issuer,
                account,
            };
            
            match get_totp_code(&config) {
                Ok(current_code) => {
                    if answer == current_code {
                        println!("✅ Correct! You solved the TOTP puzzle!");
                        true
                    } else {
                        println!("❌ Wrong, the stone tablet's glow dims...");
                        false
                    }
                }
                Err(e) => {
                    println!("❌ Failed to get TOTP code: {}", e);
                    false
                }
            }
        }
        "2" => {
            // Custom fixed question
            let question = "What is the capital city of China?";
            let correct_answer = "Beijing";
            println!("\n--- Puzzle Two: Custom Fixed Question ---");
            println!("{}", question);
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let answer = input.trim();
            if answer == correct_answer {
                println!("✅ Correct! You solved the custom puzzle!");
                true
            } else {
                println!("❌ Wrong, incorrect answer, the stone tablet's glow dims...");
                false
            }
        }
        _ => {
            println!("Invalid selection.");
            false
        }
    }
}
