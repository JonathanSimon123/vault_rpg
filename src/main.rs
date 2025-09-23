mod storage;
mod puzzles;
mod totp;
mod vault;

use crate::storage::{encrypt_mnemonic, decrypt_mnemonic};
use crate::puzzles::run_puzzle_game;
use crate::vault::Vault;
use crate::totp::{TotpConfig, generate_totp_secret, get_totp_code, generate_secret};
use clap::{Parser, Subcommand};
use totp_rs::Algorithm;


#[derive(Parser)]
#[command(name = "Vault RPG")]
#[command(about = "A mnemonic vault with puzzle games", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum TotpCommands {
    /// Generate TOTP secret and QR code
    Generate {
        /// Account name (can be set via VAULT_TOTP_ACCOUNT env var)
        #[arg(short, long)]
        account: Option<String>,
        /// Issuer name (can be set via VAULT_TOTP_ISSUER env var)
        #[arg(short, long)]
        issuer: Option<String>,
        /// Secret key (can be set via VAULT_TOTP_SECRET env var, auto-generated if not provided)
        #[arg(short, long)]
        secret: Option<String>,
        /// Algorithm type (SHA1, SHA256, SHA512)
        #[arg(long, default_value = "SHA1")]
        algorithm: String,
        /// Number of digits
        #[arg(short, long, default_value = "6")]
        digits: u8,
        /// Time step (seconds)
        #[arg(long, default_value = "30")]
        step: u64,
    },
    /// Get current TOTP verification code
    Code {
        /// Account name (can be set via VAULT_TOTP_ACCOUNT env var)
        #[arg(short, long)]
        account: Option<String>,
        /// Issuer name (can be set via VAULT_TOTP_ISSUER env var)
        #[arg(short, long)]
        issuer: Option<String>,
        /// Secret key (can be set via VAULT_TOTP_SECRET env var)
        #[arg(short, long)]
        secret: Option<String>,
        /// Algorithm type (SHA1, SHA256, SHA512)
        #[arg(long, default_value = "SHA1")]
        algorithm: String,
        /// Number of digits
        #[arg(short, long, default_value = "6")]
        digits: u8,
        /// Time step (seconds)
        #[arg(long, default_value = "30")]
        step: u64,
    },
}

#[derive(Subcommand)]
enum Commands {
    /// Interactive menu
    Menu,

    /// Create new vault
    Create {
        name: String,
        #[arg(short, long)]
        mnemonic: String,
        #[arg(short, long)]
        passphrase: String,
    },

    /// List all vaults
    List,

    /// Delete vault
    Delete {
        name: String,
    },

    /// Unlock vault
    Unlock {
        name: String,
        #[arg(short, long)]
        passphrase: String,
    },

    /// TOTP related commands
    Totp {
        #[command(subcommand)]
        totp_command: TotpCommands,
    },
}

fn parse_algorithm(alg_str: &str) -> Result<Algorithm, String> {
    match alg_str.to_uppercase().as_str() {
        "SHA1" => Ok(Algorithm::SHA1),
        "SHA256" => Ok(Algorithm::SHA256),
        "SHA512" => Ok(Algorithm::SHA512),
        _ => Err(format!("Unsupported algorithm type: {}. Supported algorithms: SHA1, SHA256, SHA512", alg_str)),
    }
}

fn show_menu() {
    println!("🎮 Vault RPG - 助记词保险库与解谜游戏");
    println!("=====================================");
    println!("");
    println!("请选择操作：");
    println!("1. 创建新保险库");
    println!("2. 解锁保险库");
    println!("3. 列出保险库");
    println!("4. 删除保险库");
    println!("5. 生成TOTP QR码");
    println!("6. 退出");
    println!("");
}

fn interactive_menu() {
    loop {
        show_menu();
        print!("请输入选择 (1-6): ");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();
        
        match choice {
            "1" => {
                println!("");
                print!("请输入保险库名称: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();
                
                print!("请输入助记词: ");
                io::stdout().flush().unwrap();
                let mut mnemonic = String::new();
                io::stdin().read_line(&mut mnemonic).unwrap();
                let mnemonic = mnemonic.trim().to_string();
                
                print!("请设置主密码: ");
                io::stdout().flush().unwrap();
                let mut passphrase = String::new();
                io::stdin().read_line(&mut passphrase).unwrap();
                let passphrase = passphrase.trim().to_string();
                
                create_vault(&name, &mnemonic, &passphrase);
            }
            "2" => {
                println!("");
                print!("请输入保险库名称: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();
                
                print!("请输入主密码: ");
                io::stdout().flush().unwrap();
                let mut passphrase = String::new();
                io::stdin().read_line(&mut passphrase).unwrap();
                let passphrase = passphrase.trim().to_string();
                
                unlock_vault(&name, &passphrase);
            }
            "3" => {
                list_vaults();
            }
            "4" => {
                println!("");
                print!("请输入要删除的保险库名称: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();
                
                delete_vault(&name);
            }
            "5" => {
                generate_totp_qr();
            }
            "6" => {
                println!("👋 再见！");
                break;
            }
            _ => {
                println!("❌ 无效选择，请重新输入");
            }
        }
        println!("");
        print!("按 Enter 继续...");
        io::stdout().flush().unwrap();
        let mut _input = String::new();
        io::stdin().read_line(&mut _input).unwrap();
        println!("");
    }
}

fn create_vault(name: &str, mnemonic: &str, passphrase: &str) {
    let vault = Vault::new(name);
    if vault.exists() {
        println!("保险库 '{}' 已存在！", vault.name);
        return;
    }
    match encrypt_mnemonic(mnemonic, passphrase) {
        Ok(ciphertext) => {
            if let Err(e) = vault.save(ciphertext.as_bytes()) {
                eprintln!("保存保险库文件失败: {}", e);
                return;
            }
            println!("✅ 助记词已加密并保存到保险库 '{}' 中。", vault.name);
        }
        Err(e) => eprintln!("加密失败: {}", e),
    }
}

fn unlock_vault(name: &str, passphrase: &str) {
    let vault = Vault::new(name);
    if !vault.exists() {
        println!("指定的保险库 '{}' 不存在。", vault.name);
        return;
    }
    println!("💀 你回到了这个地牢...");
    if run_puzzle_game() {
        match vault.load() {
            Ok(ciphertext) => match decrypt_mnemonic(&ciphertext, passphrase) {
                Ok(mnemonic) => println!("\n🎉 解锁成功！你的助记词是:\n{}", mnemonic),
                Err(e) => eprintln!("\n❌ 解密失败: {}", e),
            },
            Err(e) => eprintln!("读取保险库文件失败: {}", e),
        }
    } else {
        println!("\n❌ 你未能解开谜题，宝藏仍沉睡在地牢深处...");
    }
}

fn list_vaults() {
    match Vault::list_vaults() {
        Ok(vaults) => {
            if vaults.is_empty() {
                println!("未找到保险库。");
            } else {
                println!("所有保险库:");
                for v in vaults {
                    println!("- {}", v);
                }
            }
        }
        Err(e) => eprintln!("读取保险库列表失败: {}", e),
    }
}

fn delete_vault(name: &str) {
    let vault = Vault::new(name);
    if vault.exists() {
        match vault.delete() {
            Ok(_) => println!("保险库 '{}' 已删除。", vault.name),
            Err(e) => println!("删除失败: {}", e),
        }
    } else {
        println!("指定的保险库 '{}' 不存在。", vault.name);
    }
}

fn generate_totp_qr() {
    let secret_key = std::env::var("VAULT_TOTP_SECRET").unwrap_or_else(|_| {
        println!("🔑 生成密钥: JBSWY3DPEHPK3PXP");
        "JBSWY3DPEHPK3PXP".to_string()
    });
    
    let account_name = std::env::var("VAULT_TOTP_ACCOUNT").unwrap_or_else(|_| {
        "demo@example.com".to_string()
    });
    
    let issuer_name = std::env::var("VAULT_TOTP_ISSUER").ok();
    
    let config = TotpConfig {
        algorithm: Algorithm::SHA1,
        digits: 6,
        skew: 1,
        step: 30,
        secret: secret_key,
        issuer: issuer_name,
        account: account_name,
    };
    
    match generate_totp_secret(&config) {
        Ok(_) => println!("✅ TOTP配置已成功生成！"),
        Err(e) => eprintln!("❌ 生成TOTP失败: {}", e),
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Menu => {
            interactive_menu();
        }
        Commands::Create { name, mnemonic, passphrase } => {
            let vault = Vault::new(name);
            if vault.exists() {
                println!("Vault '{}' already exists!", vault.name);
                return;
            }
            match encrypt_mnemonic(mnemonic, passphrase) {
                Ok(ciphertext) => {
                    if let Err(e) = vault.save(ciphertext.as_bytes()) {
                        eprintln!("Failed to save vault file: {}", e);
                        return;
                    }
                    println!("✅ Mnemonic has been encrypted and saved in vault '{}'.", vault.name);
                }
                Err(e) => eprintln!("Encryption failed: {}", e),
            }
        }

        Commands::List => match Vault::list_vaults() {
            Ok(vaults) => {
                if vaults.is_empty() {
                    println!("No vaults found.");
                } else {
                    println!("All vaults:");
                    for v in vaults {
                        println!("- {}", v);
                    }
                }
            }
            Err(e) => eprintln!("Failed to read vault list: {}", e),
        },

        Commands::Delete { name } => {
            let vault = Vault::new(name);
            if vault.exists() {
                match vault.delete() {
                    Ok(_) => println!("Vault '{}' has been deleted.", vault.name),
                    Err(e) => println!("Deletion failed: {}", e),
                }
            } else {
                println!("The specified vault '{}' does not exist.", vault.name);
            }
        }

        Commands::Unlock { name, passphrase } => {
            let vault = Vault::new(name);
            if !vault.exists() {
                println!("The specified vault '{}' does not exist.", vault.name);
                return;
            }
            println!("💀 You have returned to this dungeon...");
            if run_puzzle_game() {
                match vault.load() {
                    Ok(ciphertext) => match decrypt_mnemonic(&ciphertext, passphrase) {
                        Ok(mnemonic) => println!("\n🎉 Unlock successful! Your mnemonic is:\n{}", mnemonic),
                        Err(e) => eprintln!("\n❌ Decryption failed: {}", e),
                    },
                    Err(e) => eprintln!("Failed to read vault file: {}", e),
                }
            } else {
                println!("\n❌ You failed to solve the puzzle, the treasure still sleeps deep in the dungeon...");
            }
        }

        Commands::Totp { totp_command } => {
            match totp_command {
                TotpCommands::Generate { account, issuer, secret, algorithm, digits, step } => {
                    let alg = match parse_algorithm(algorithm) {
                        Ok(alg) => alg,
                        Err(e) => {
                            eprintln!("❌ {}", e);
                            return;
                        }
                    };

                    // 优先使用命令行参数，否则从环境变量获取默认值
                    let secret_key = secret.clone().unwrap_or_else(|| {
                        std::env::var("VAULT_TOTP_SECRET").unwrap_or_else(|_| {
                            let generated = generate_secret();
                            println!("🔑 Generated secret: {}", generated);
                            generated
                        })
                    });

                    let account_name = account.clone().unwrap_or_else(|| {
                        std::env::var("VAULT_TOTP_ACCOUNT").unwrap_or_else(|_| {
                            eprintln!("❌ TOTP account not provided. Please set VAULT_TOTP_ACCOUNT environment variable or use --account parameter.");
                            std::process::exit(1);
                        })
                    });
                    let issuer_name = issuer.clone().or_else(|| std::env::var("VAULT_TOTP_ISSUER").ok());

                    let config = TotpConfig {
                        algorithm: alg,
                        digits: *digits,
                        skew: 1,
                        step: *step,
                        secret: secret_key,
                        issuer: issuer_name,
                        account: account_name,
                    };

                    match generate_totp_secret(&config) {
                        Ok(_) => println!("✅ TOTP configuration has been generated successfully!"),
                        Err(e) => eprintln!("❌ Failed to generate TOTP: {}", e),
                    }
                }

                TotpCommands::Code { account, issuer, secret, algorithm, digits, step } => {
                    let alg = match parse_algorithm(algorithm) {
                        Ok(alg) => alg,
                        Err(e) => {
                            eprintln!("❌ {}", e);
                            return;
                        }
                    };

                    // 优先使用命令行参数，否则从环境变量获取默认值
                    let secret_key = secret.clone().unwrap_or_else(|| {
                        std::env::var("VAULT_TOTP_SECRET").unwrap_or_else(|_| {
                            eprintln!("❌ TOTP secret not provided. Please set VAULT_TOTP_SECRET environment variable or use --secret parameter.");
                            std::process::exit(1);
                        })
                    });

                    let account_name = account.clone().unwrap_or_else(|| {
                        std::env::var("VAULT_TOTP_ACCOUNT").unwrap_or_else(|_| {
                            eprintln!("❌ TOTP account not provided. Please set VAULT_TOTP_ACCOUNT environment variable or use --account parameter.");
                            std::process::exit(1);
                        })
                    });

                    let issuer_name = issuer.clone().or_else(|| std::env::var("VAULT_TOTP_ISSUER").ok());

                    let config = TotpConfig {
                        algorithm: alg,
                        digits: *digits,
                        skew: 1,
                        step: *step,
                        secret: secret_key,
                        issuer: issuer_name,
                        account: account_name,
                    };

                    match get_totp_code(&config) {
                        Ok(code) => println!("🔐 Current verification code: {}", code),
                        Err(e) => eprintln!("❌ Failed to get verification code: {}", e),
                    }
                }
            }
        }
    }
}