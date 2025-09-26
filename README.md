# CipherLink
Ephemeral redirector with key-gated access built in Rust.

⚠️ This is **not** a zero-knowledge system. The server sees the plaintext URL during encryption. See threat model below.

## Overview / Motivation
The practicality of this is for users to create an obfuscated shareable link. So they could share a link like ```/decrypt/{id}/{key}```, where key would only be known to those that the user wants to grant access. The link only works once, and is then deleted from this app's database.

I created this because I wanted to know what it was like to create a REST app in Rust.

I had hopes of deploying this to AWS using DynamoDB, API Gateway, and Lambda, but unfortunately AWS changed their free tier model.

So instead, the app works locally with a Docker instance of DynamoDB and has a Lambda mode where the app runs with cargo-lambda simulating Lambda invocations.

## Quickstart / Installation
### Usage
Starting this app should be done through the makefile. When starting the app, first build the DynamoDB database, then seed it. Seeding it actually scaffolds the tables, as well as puts dummy data into the DB.
```
make run-db
make seed
```
To run the REST version of this app:
```
make server
```
One can use the bruno collection in the docs folder to interact with the app.
To simulate running in Lambda:
```
make lambda
```
see makefile commands for the available Lambda commands to interact with this app in that mode.
### Config
What little environment variables there are, exist at the top of the makefile.
### Testing 
Unit tests are pretty minimal, tests instead focus on behavior rather than coverage. Depending on the app mode, one can run integration tests for REST or Lambda mode:
```
make test-rest
```
or
```
make test-lambda
```

## Other
Please see the github wiki for additional information.

## Threat Model
* **What it protects**: casual scraping, bot access, link obfuscation

* **What it doesn’t**: server compromise, true zero-knowledge

* **Why**: encryption is done server-side for simplicity and demonstration