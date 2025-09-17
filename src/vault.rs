#[derive(Debug, Clone)]
pub struct Vault {
    pub name: String,
    pub path: String,
}


impl Vault {
    pub fn new(name: &str) -> Self {
        let path = format!("secrets/vault_{}.enc", name);
        Vault {
            name: name.to_string(),
            path,
        }
    }

    pub fn exists(&self) -> bool {
        std::path::Path::new(&self.path).exists()
    }

    pub fn save(&self, ciphertext: &[u8]) -> std::io::Result<()> {
        std::fs::create_dir_all("secrets")?;
        std::fs::write(&self.path, ciphertext)
    }

    pub fn load(&self) -> std::io::Result<Vec<u8>> {
        std::fs::read(&self.path)
    }

    /// 列出所有保险柜名称（去除扩展名和前缀）
    pub fn list_vaults() -> std::io::Result<Vec<String>> {
        let mut vaults = Vec::new();
        let dir = std::fs::read_dir("secrets")?;
        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
                if fname.starts_with("vault_") && fname.ends_with(".enc") {
                    let name = &fname[6..fname.len() - 4];
                    vaults.push(name.to_string());
                }
            }
        }
        Ok(vaults)
    }

    /// 删除该保险柜文件
    pub fn delete(&self) -> std::io::Result<()> {
        if self.exists() {
            std::fs::remove_file(&self.path)
        } else {
            Ok(())
        }
    }
}