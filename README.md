# CipherLink
Ephemeral redirector with key-gated access built in Rust.

⚠️ This is **not** a zero-knowledge system. The server sees the plaintext URL during encryption. See [threat model](https://github.com/travis-james/CipherLink/blob/main/README.md#threat-model) below.

## Overview
The practicality of this is for users to create an obfuscated shareable link that only works with a key, and will become invalid after one use.
Essentially the flow is:
#### /encrypt
```
[Client] → (plaintext + key) → [Server]

[Server] → encrypt(plaintext, key) → store in DB → return UUID
```

#### /decrypt
```
[Client] → (UUID + key) → [Server]

[Server] → fetch encrypted → decrypt with key → validate URL → redirect
```

## Motivation

I created this because I wanted to know what it was like to create a REST app in Rust.

I had hopes of deploying this to AWS using DynamoDB, API Gateway, and Lambda, but unfortunately AWS changed their free tier model.

So instead, the app works locally with a Docker instance of DynamoDB and has a Lambda mode where the app runs with cargo-lambda simulating Lambda invocations.

## Quickstart / Installation
### Usage
Starting this app should be done through the [Makefile](https://github.com/travis-james/CipherLink/blob/main/Makefile). When starting the app, first build the DynamoDB database, then seed it. Seeding it actually scaffolds the tables, as well as puts dummy data into the DB.
```
make run-db
make seed
```
To run the REST version of this app:
```
make server
```
One can use the [bruno collection in the docs folder](https://github.com/travis-james/CipherLink/tree/main/docs/cipherlink) to interact with the app.
To simulate running in Lambda:
```
make lambda
```
see Makefile commands for the available Lambda commands to interact with this app in that mode.
### Config
See [.env](https://github.com/travis-james/CipherLink/blob/main/.env) file.
Docker variables are at the top of the [Makefile](https://github.com/travis-james/CipherLink/blob/3d067076f8c503fde5ca0fcea8e5d42be1aa23a1/Makefile#L1-L4) for now.
### Testing 
Unit tests are pretty minimal, tests instead focus on behavior rather than coverage. Depending on the app mode, one can run integration tests for REST or Lambda mode:
```
make test-rest
```
or
```
make test-lambda
```

## Threat Model
* **What it protects**: casual scraping, bot access, link obfuscation

* **What it doesn’t**: server compromise, true zero-knowledge

* **Why**: encryption is done server-side for simplicity and demonstration

## Other
Please see the github [wiki](https://github.com/travis-james/CipherLink/wiki) for additional information.
