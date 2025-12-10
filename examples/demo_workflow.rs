//! Demo workflow: encrypt and decrypt a message using the AegisKey crypto module.
//!
//! Run with:
//!     cargo run --example demo_workflow

use aegiskey_orchestrator::crypto;

fn main() {
    println!("═══ AegisKey-Orchestrator — Demo Workflow ═══");
    println!();

    let password = "demo_password_do_not_use_in_prod";
    let message = b"Hello from AegisKey-Orchestrator!\n\
                    This is a demonstration of the encrypt/decrypt pipeline.\n\
                    Malwarius Infrastructure Team - 2026.";

    println!("1. Original message ({} bytes):", message.len());
    println!("   {}", String::from_utf8_lossy(message));
    println!();

    // Encrypt
    let encrypted = crypto::encrypt(message, password).expect("Encryption failed");
    println!("2. Encrypted: {} bytes (salt + nonce + ciphertext)", encrypted.len());
    println!("   First 32 bytes (hex): {}", hex_preview(&encrypted, 32));
    println!();

    // Decrypt
    let decrypted = crypto::decrypt(&encrypted, password).expect("Decryption failed");
    println!("3. Decrypted ({} bytes):", decrypted.len());
    println!("   {}", String::from_utf8_lossy(&decrypted));
    println!();

    // Verify
    assert_eq!(
        message.to_vec(),
        decrypted,
        "Round-trip failed — something is very wrong"
    );
    println!("✔ Round-trip successful — plaintext matches.");
    println!();

    // Demonstrate wrong password
    print!("4. Attempting decryption with wrong password... ");
    match crypto::decrypt(&encrypted, "wrong_password") {
        Ok(_) => println!("Unexpectedly succeeded (this is bad)"),
        Err(e) => println!("correctly failed: {}", e),
    }

    println!();
    println!("═══ Demo complete ═══");
}

/// Format the first `n` bytes of `data` as hex for display.
fn hex_preview(data: &[u8], n: usize) -> String {
    data.iter()
        .take(n)
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(" ")
}
