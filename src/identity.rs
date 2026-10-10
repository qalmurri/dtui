use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};

use directories::ProjectDirs;
use ed25519_dalek::SigningKey;
use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityMode {
    Anonymous,
    Persistent,
}

#[derive(Serialize, Deserialize)]
struct StoredIdentity {
    version: u32,
    alias: String,
    secret_key: String,
    public_key: String,
}

pub enum Identity {
    Anonymous {
        session_id: [u8; 32],
        alias: String,
    },
    Persistent {
        signing_key: SigningKey,
        alias: String,
    },
}

impl Identity {
    /// Memuat identitas persisten jika tersedia.
    /// Jika belum ada, buat sesi anonymous baru.
    ///
    /// Jika file identitas rusak, fungsi mengembalikan error
    /// dan tidak diam-diam mengganti identitas pengguna.
    pub fn load_or_anonymous() -> Result<Self, Box<dyn Error>> {
        let path = identity_path()?;

        if path.exists() {
            return Self::load_persistent(&path);
        }

        Ok(Self::new_anonymous())
    }

    /// Membuat identitas anonymous yang berlaku selama sesi.
    pub fn new_anonymous() -> Self {
        let mut session_id = [0_u8; 32];
        OsRng.fill_bytes(&mut session_id);

        let alias = format!(
            "Anonymous-{:02X}{:02X}{:02X}",
            session_id[0], session_id[1], session_id[2],
        );

        Self::Anonymous { session_id, alias }
    }

    /// Membuat dan menyimpan identitas persisten.
    ///
    /// File tidak akan menimpa identitas yang sudah ada.
    pub fn create_persistent(alias: &str) -> Result<Self, Box<dyn Error>> {
        let alias = validate_alias(alias)?;
        let path = identity_path()?;

        fs::create_dir_all(path.parent().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Invalid identity path")
        })?)?;

        let signing_key = SigningKey::generate(&mut OsRng);
        let public_key = signing_key.verifying_key();

        let stored = StoredIdentity {
            version: 1,
            alias: alias.clone(),
            secret_key: hex::encode(signing_key.to_bytes()),
            public_key: hex::encode(public_key.to_bytes()),
        };

        let contents = toml::to_string_pretty(&stored)?;

        let mut options = OpenOptions::new();
        options.write(true).create_new(true);

        // Di Unix, hanya pemilik file yang memiliki izin akses.
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }

        let mut file = options.open(&path)?;

        if let Err(error) = file.write_all(contents.as_bytes()) {
            drop(file);
            let _ = fs::remove_file(&path);
            return Err(error.into());
        }

        if let Err(error) = file.sync_all() {
            drop(file);
            let _ = fs::remove_file(&path);
            return Err(error.into());
        }

        Ok(Self::Persistent { signing_key, alias })
    }

    /// Memuat identitas persisten dan memverifikasi pasangan kuncinya.
    pub fn load_persistent(path: &Path) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let stored: StoredIdentity = toml::from_str(&contents)?;

        if stored.version != 1 {
            return Err(format!("Unsupported identity file version: {}", stored.version).into());
        }

        let alias = validate_alias(&stored.alias)?;

        let secret_bytes: [u8; 32] = hex::decode(&stored.secret_key)?
            .try_into()
            .map_err(|_| "Private key harus berukuran 32 byte")?;

        let public_bytes: [u8; 32] = hex::decode(&stored.public_key)?
            .try_into()
            .map_err(|_| "Public key harus berukuran 32 byte")?;

        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let derived_public = signing_key.verifying_key();

        if derived_public.to_bytes() != public_bytes {
            return Err("Public key tidak cocok dengan private key".into());
        }

        Ok(Self::Persistent { signing_key, alias })
    }

    pub fn alias(&self) -> &str {
        match self {
            Self::Anonymous { alias, .. } | Self::Persistent { alias, .. } => alias,
        }
    }

    pub fn mode(&self) -> IdentityMode {
        match self {
            Self::Anonymous { .. } => IdentityMode::Anonymous,
            Self::Persistent { .. } => IdentityMode::Persistent,
        }
    }

    pub fn mode_label(&self) -> &'static str {
        match self.mode() {
            IdentityMode::Anonymous => "Anonymous",
            IdentityMode::Persistent => "Persistent",
        }
    }

    /// ID untuk ditampilkan di UI.
    /// Ini bukan token autentikasi server.
    pub fn display_id(&self) -> String {
        match self {
            Self::Anonymous { session_id, .. } => {
                format!("session:{}", hex::encode(&session_id[..6]))
            }
            Self::Persistent { signing_key, .. } => {
                let public_key = signing_key.verifying_key();

                format!("key:{}", hex::encode(&public_key.to_bytes()[..8]))
            }
        }
    }

    /// Mengambil public key untuk kebutuhan identitas kriptografis.
    pub fn public_key_bytes(&self) -> Option<[u8; 32]> {
        match self {
            Self::Anonymous { .. } => None,
            Self::Persistent { signing_key, .. } => Some(signing_key.verifying_key().to_bytes()),
        }
    }

    /// Menandatangani data menggunakan private key.
    /// Hanya tersedia untuk identitas persisten.
    pub fn sign(&self, message: &[u8]) -> Option<ed25519_dalek::Signature> {
        use ed25519_dalek::Signer;

        match self {
            Self::Anonymous { .. } => None,
            Self::Persistent { signing_key, .. } => Some(signing_key.sign(message)),
        }
    }
}

fn validate_alias(value: &str) -> Result<String, Box<dyn Error>> {
    let alias = value.trim();

    if alias.is_empty() || alias.chars().count() > 32 {
        return Err("Alias harus berisi 1–32 karakter".into());
    }

    if alias.chars().any(char::is_control) {
        return Err("Alias tidak boleh berisi karakter kontrol".into());
    }

    Ok(alias.to_owned())
}

pub fn identity_path() -> Result<std::path::PathBuf, Box<dyn Error>> {
    let dirs = ProjectDirs::from("com", "qalmurri", "srawung").ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Tidak dapat menentukan direktori konfigurasi",
        )
    })?;

    Ok(dirs.config_dir().join("identity.toml"))
}
