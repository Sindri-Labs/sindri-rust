# Sindri Rust SDK + zkVM Demo

This project demonstrates how to develop zkVM code on Sindri.

The Sp1 program we use is an arbitrary regular expression checker.
Our runner script applies that zkVM program to the task of validating emails.
We submit six different example emails in parallel and all of them are validated (with an execution proof!) in parallel.

## Running the Demo

Requirements: 
* [Rustup](https://www.rust-lang.org/tools/install)
* [Sindri API Key](https://sindri.app/z/me/page/settings/api-keys)

After cloning the project, you can run the zkVM via:
```
cd script
SINDRI_API_KEY=<your-key-here> cargo run --release
```

#### Example Output
```
Submitting 6 emails
 ✓ Email 'user@example.com' is valid (ZKP verified)
 ✓ Email 'invalid.email@' is invalid (ZKP verified)
 ✓ Email 'user.name+tag@gmail.com' is valid (ZKP verified)
 ✓ Email '@nouser.com' is invalid (ZKP verified)
 ✓ Email 'spaces in@email.com' is invalid (ZKP verified)
 ✓ Email 'user@sub.domain.co.uk' is valid (ZKP verified)
```

## Project Structure
* `program/`: Contains the SP1 source code of the regex validator
* `script/`: Contains the client code that executes the zkVM program through Sindri's GPU-powered proving backend
