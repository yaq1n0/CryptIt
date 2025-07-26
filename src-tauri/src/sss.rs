use shamirs::{combine, split};
use thiserror::Error;
use base64::{Engine, engine::general_purpose};

#[derive(Error, Debug)]
pub enum SSSError {
    #[error("Invalid threshold: k must be <= n and both must be > 0")]
    InvalidThreshold,
    #[error("Failed to generate shares")]
    ShareGenerationFailed,
    #[error("Failed to reconstruct secret")]
    ReconstructionFailed,
    #[error("Invalid share format")]
    InvalidShareFormat,
    #[error("Insufficient shares provided")]
    InsufficientShares,
}

pub fn split_secret(secret: &[u8], k: u8, n: u8) -> Result<Vec<String>, SSSError> {
    if k == 0 || n == 0 || k > n {
        return Err(SSSError::InvalidThreshold);
    }

    // Use the shamirs crate - much simpler API!
    let shares = split(secret, n as usize, k as usize)
        .map_err(|_| SSSError::ShareGenerationFailed)?;
    
    // Encode shares as base64 strings for easy transport
    let encoded_shares: Vec<String> = shares
        .iter()
        .map(|share| general_purpose::STANDARD.encode(share))
        .collect();

    Ok(encoded_shares)
}

pub fn reconstruct_secret(encoded_shares: &[String]) -> Result<Vec<u8>, SSSError> {
    if encoded_shares.is_empty() {
        return Err(SSSError::InsufficientShares);
    }

    // Decode base64 shares
    let shares: Result<Vec<Vec<u8>>, _> = encoded_shares
        .iter()
        .map(|encoded_share| {
            general_purpose::STANDARD
                .decode(encoded_share)
                .map_err(|_| SSSError::InvalidShareFormat)
        })
        .collect();
    
    let shares = shares?;
    
    // Use the shamirs crate to reconstruct - super simple!
    let secret = combine(&shares)
        .map_err(|_| SSSError::ReconstructionFailed)?;

    Ok(secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_reconstruct() {
        let secret = b"this is a secret message";
        let k = 2;
        let n = 3;
        
        let shares = split_secret(secret, k, n).unwrap();
        assert_eq!(shares.len(), n as usize);
        
        // Test with minimum shares
        let reconstructed = reconstruct_secret(&shares[0..k as usize]).unwrap();
        assert_eq!(secret, reconstructed.as_slice());
        
        // Test with all shares
        let reconstructed = reconstruct_secret(&shares).unwrap();
        assert_eq!(secret, reconstructed.as_slice());
    }
    
    #[test]
    fn test_insufficient_shares() {
        let secret = b"secret";
        let shares = split_secret(secret, 3, 5).unwrap();
        
        // Try with only 1 share when 3 are required
        let result = reconstruct_secret(&shares[0..1]);
        // With the shamirs crate, this should properly fail
        assert!(result.is_err(), "Should fail with insufficient shares");
    }
} 