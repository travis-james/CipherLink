# CipherLink
Ephemeral redirector with key-gated access built in Rust

⚠️ This is solely a personal project for myself; I was curious to know what it was like to build a web service in Rust. It is **not** a zero-knowledge system. The server sees the plaintext URL during encryption. See threat model below.

## Threat Model
* **What it protects**: casual scraping, bot access, link obfuscation

* **What it doesn’t**: server compromise, true zero-knowledge

* **Why**: encryption is done server-side for simplicity and demonstration