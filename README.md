# XOR Cipher in Rust

This is a simple implementation of the XOR cipher in Rust. The XOR cipher is a symmetric encryption algorithm where each
character in the plaintext is bitwise XORed with a corresponding character in the key. This implementation allows you to
encrypt and decrypt strings using a password.

## Usage

1. Clone the repository:
   ```bash
   git clone https://github.com/your_username/xor_cipher_rust.git

## Example

### Encryption

```
Enter your string
Hello, World!
Enter your password
password123
Cipher string:
[119, 52, 120, 6, 43, 42, 63, 120, 50, 9, 56, 52, 32, 21, 15]
``` 

### Decryption

```
Enter your cipher data:
119, 52, 120, 6, 43, 42, 63, 120, 50, 9, 56, 52, 32, 21, 15
Enter the password:
password123
Initial string "Hello, World!"

```

## Dependencies

- **bytes**: This crate provides utilities for working with byte sequences.
- **clippy**: A collection of lints to catch common mistakes and improve code consistency.