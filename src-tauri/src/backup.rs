use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key};
use argon2::Argon2;
use rand::rngs::OsRng;
use rand::RngCore;
use rusqlite::Connection;

const MAGIC: &[u8; 8] = b"TURGITE1";

/// Produce a consistent snapshot of the live DB as raw bytes.
pub fn snapshot_bytes(conn: &Connection) -> Result<Vec<u8>, String> {
    let tmp = std::env::temp_dir().join(format!("turgite-snap-{}.db", std::process::id()));
    let _ = std::fs::remove_file(&tmp);
    conn.execute("VACUUM INTO ?1", [tmp.to_str().unwrap()])
        .map_err(|e| e.to_string())?;
    let bytes = std::fs::read(&tmp).map_err(|e| e.to_string())?;
    let _ = std::fs::remove_file(&tmp);
    Ok(bytes)
}

pub fn encrypt(bytes: &[u8], password: &str) -> Result<Vec<u8>, String> {
    let mut salt = [0u8; 16];
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce_bytes);

    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), &salt, &mut key)
        .map_err(|e| e.to_string())?;

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
    let nonce = aes_gcm::aead::Nonce::<Aes256Gcm>::from_slice(&nonce_bytes);
    let ct = cipher
        .encrypt(nonce, bytes)
        .map_err(|e| e.to_string())?;

    let mut out = Vec::with_capacity(MAGIC.len() + salt.len() + nonce_bytes.len() + ct.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ct);
    Ok(out)
}

pub fn decrypt(data: &[u8], password: &str) -> Result<Vec<u8>, String> {
    if data.len() < MAGIC.len() + 16 + 12 + 16 {
        return Err("invalid backup file".into());
    }
    let (magic, rest) = data.split_at(MAGIC.len());
    if magic != MAGIC {
        return Err("backup is not encrypted".into());
    }
    let salt = &rest[0..16];
    let nonce_bytes = &rest[16..28];
    let ct = &rest[28..];

    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| e.to_string())?;

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
    let nonce = aes_gcm::aead::Nonce::<Aes256Gcm>::from_slice(nonce_bytes);
    cipher.decrypt(nonce, ct).map_err(|e| e.to_string())
}

/// Replace the live connection's contents with the (decrypted) sqlite bytes.
pub fn restore_into(conn: &mut Connection, db_bytes: &[u8]) -> Result<(), String> {
    let tmp = std::env::temp_dir().join(format!("turgite-restore-{}.db", std::process::id()));
    std::fs::write(&tmp, db_bytes).map_err(|e| e.to_string())?;
    let src = Connection::open(&tmp).map_err(|e| e.to_string())?;
    let backup = rusqlite::backup::Backup::new(&src, conn).map_err(|e| e.to_string())?;
    backup.run_to_completion(64, std::time::Duration::from_millis(5), None)
        .map_err(|e| e.to_string())?;
    let _ = std::fs::remove_file(&tmp);
    Ok(())
}
