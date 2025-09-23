# Vault RPG

Vault RPG is a command-line tool that combines a mnemonic vault with puzzle games. You can safely store your mnemonics and unlock the vault by solving puzzles.

## Features

- Multi-vault management with independent encrypted storage for each vault
- Mnemonic encryption and decryption to protect your private information
- TOTP QR code generation supporting Google Authenticator and other dynamic code apps
- Puzzle games that must be solved before unlocking vault contents

## Quick Start

1. **Install Dependencies**

   ```bash
   cargo build
   ```
2. **Configure TOTP (Optional)**

   For TOTP puzzle functionality, set environment variables:

   ```bash
   export VAULT_TOTP_SECRET="your_base32_secret_key"
   export VAULT_TOTP_ACCOUNT="your@email.com"  # Optional
   export VAULT_TOTP_ISSUER="YourService"      # Optional
   ```
3. **Run the Project**

   ```bash
   cargo run
   ```
4. **Create or Select a Vault**

   Follow the prompts to enter vault name, set mnemonic and master password.
5. **Solve Puzzles and Unlock Vault**

   Complete puzzle challenges and enter master password to unlock mnemonics.

## Main Commands

- Create new vault
- Generate TOTP QR codes
- Unlock vault and display mnemonics

## Future Development Plans

- [X] Vault listing and deletion functionality: Support listing all vaults and deleting unnecessary vaults.
- [X] Automatic mnemonic validation: Automatically detect format and validity when entering mnemonics.
- [ ] Multiple puzzle types: Add more puzzle gameplay to increase unlocking fun.
- [ ] Vault import/export: Support vault data import and export for easy backup and migration.
- [ ] Graphical interface (GUI): Develop desktop graphical interface to improve user experience.
- [ ] Multi-user support: Allow different users to independently manage their own vaults.
- [ ] Security enhancements: Support hardware keys, fingerprints and other multi-factor authentication methods.
- [ ] Logging and auditing: Record vault operation logs to improve security traceability.

## License

MIT
