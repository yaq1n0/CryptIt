use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

mod crypto;
mod sss;

use crypto::{EncryptionKey, encrypt_data, decrypt_data};
use sss::{split_secret, reconstruct_secret};

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptionResult {
    pub shares: Vec<String>,
    pub encrypted_file_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptionResult {
    pub output_path: String,
}

#[tauri::command]
async fn encrypt_file(
    file_path: String,
    output_dir: String,
    k: u8,
    n: u8,
) -> Result<EncryptionResult, String> {
    println!("Encrypting file: {} to directory: {} with {}-of-{} sharing", file_path, output_dir, k, n);
    
    // Read the input file
    let file_data = fs::read(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    // Generate encryption key
    let key = EncryptionKey::generate();
    
    // Encrypt the file data
    let encrypted_data = encrypt_data(&file_data, &key)
        .map_err(|e| format!("Encryption failed: {}", e))?;
    
    // Split the key using Shamir Secret Sharing
    let shares = split_secret(key.as_bytes(), k, n)
        .map_err(|e| format!("Failed to generate shares: {}", e))?;
    
    // Create output file path
    let input_path = Path::new(&file_path);
    let file_name = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("encrypted");
    let output_path = PathBuf::from(&output_dir).join(format!("{}.cryptit", file_name));
    
    // Create a simple file format: [nonce][ciphertext]
    let mut file_content = Vec::new();
    file_content.extend_from_slice(&encrypted_data.nonce);
    file_content.extend_from_slice(&encrypted_data.ciphertext);
    
    // Write encrypted file
    fs::write(&output_path, &file_content)
        .map_err(|e| format!("Failed to write encrypted file: {}", e))?;
    
    Ok(EncryptionResult {
        shares,
        encrypted_file_path: output_path.to_string_lossy().to_string(),
    })
}

#[tauri::command]
async fn decrypt_file(
    file_path: String,
    output_dir: String,
    shares: Vec<String>,
) -> Result<DecryptionResult, String> {
    println!("Decrypting file: {} to directory: {} with {} shares", file_path, output_dir, shares.len());
    
    // Read the encrypted file
    let encrypted_file_data = fs::read(&file_path)
        .map_err(|e| format!("Failed to read encrypted file: {}", e))?;
    
    // Parse the file format: [nonce][ciphertext]
    if encrypted_file_data.len() < 12 {
        return Err("Invalid encrypted file format".to_string());
    }
    
    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&encrypted_file_data[0..12]);
    let ciphertext = encrypted_file_data[12..].to_vec();
    
    let encrypted_data = crypto::EncryptedData {
        nonce,
        ciphertext,
    };
    
    // Reconstruct the key from shares
    let key_bytes = reconstruct_secret(&shares)
        .map_err(|e| format!("Failed to reconstruct key: {}", e))?;
    
    let key = EncryptionKey::from_bytes(&key_bytes)
        .map_err(|e| format!("Invalid key: {}", e))?;
    
    // Decrypt the data
    let decrypted_data = decrypt_data(&encrypted_data, &key)
        .map_err(|e| format!("Decryption failed: {}", e))?;
    
    // Create output file path
    let input_path = Path::new(&file_path);
    let file_name = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("decrypted");
    
    // Remove .cryptit extension if present
    let clean_name = if file_name.ends_with(".cryptit") {
        &file_name[..file_name.len() - 8]
    } else {
        file_name
    };
    
    let output_path = PathBuf::from(&output_dir).join(format!("{}_decrypted.txt", clean_name));
    
    // Write decrypted file
    fs::write(&output_path, &decrypted_data)
        .map_err(|e| format!("Failed to write decrypted file: {}", e))?;
    
    Ok(DecryptionResult {
        output_path: output_path.to_string_lossy().to_string(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![encrypt_file, decrypt_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
