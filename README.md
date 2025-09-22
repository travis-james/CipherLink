# CipherLink
Ephemeral redirector with key-gated access built in Rust

⚠️ This is a resume project to demonstrate Rust backend skills. It is **not** a zero-knowledge system. The server sees the plaintext URL during encryption. See threat model below.

## Threat Model
* **What it protects**: casual scraping, bot access, link obfuscation

* **What it doesn’t**: server compromise, true zero-knowledge

* **Why**: encryption is done server-side for simplicity and demonstration