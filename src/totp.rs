use totp_rs::{Algorithm, TOTP, Secret};
use base64::{Engine as _, engine::general_purpose};

pub struct TotpConfig {
    pub algorithm: Algorithm,
    pub digits: u8,
    pub skew: u8,
    pub step: u64,
    pub secret: String,
    pub issuer: Option<String>,
    pub account: String,
}

impl Default for TotpConfig {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::SHA1,
            digits: 6,
            skew: 1,
            step: 30,
            secret: String::new(),
            issuer: None,
            account: String::new(),
        }
    }
}

pub fn generate_totp_secret(config: &TotpConfig) -> Result<(), Box<dyn std::error::Error>> {
    let secret_bytes = Secret::Encoded(config.secret.clone()).to_bytes()?;
    let totp = TOTP::new(
        config.algorithm,
        config.digits.into(),
        config.skew,
        config.step,
        secret_bytes,
        config.issuer.clone(),
        config.account.clone(),
    )?;
    
    match totp.get_qr_base64() {
        Ok(base64) => {
            let png_bytes = general_purpose::STANDARD.decode(base64)?;
            let filename = format!("totp_{}.png", config.account);
            std::fs::write(&filename, png_bytes)?;
            println!("✅ QR code saved as: {}", filename);
            Ok(())
        },
        Err(e) => {
            eprintln!("❌ Failed to generate QR code: {}", e);
            Err(e.into())
        },
    }
}

pub fn get_totp_code(config: &TotpConfig) -> Result<String, Box<dyn std::error::Error>> {
    let secret_bytes = Secret::Encoded(config.secret.clone()).to_bytes()?;
    let totp = TOTP::new(
        config.algorithm,
        config.digits.into(),
        config.skew,
        config.step,
        secret_bytes,
        config.issuer.clone(),
        config.account.clone(),
    )?;
    
    let current_code = totp.generate_current()?;
    Ok(current_code.to_string())
}

pub fn generate_secret() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut rng = rand::thread_rng();
    
    (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}