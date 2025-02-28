# Email Sender

A simple Rust application for sending emails using the lettre library.

## Setup

1. Create a `.env` file in the root directory with the following variables:
   ```
   APP_PASSWORD="your_app_password"
   SENDER="your_email@example.com"
   ```

2. Create a `recipients.json` file with your recipient list (this file is gitignored)

3. Install Rust if you haven't already:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

## Dependencies

- lettre (0.11): For email functionality
- serde (1.0): For JSON serialization/deserialization
- serde_json (1.0): For working with JSON files

## Building and Running

1. Build the project:
   ```
   cargo build
   ```

2. Run the application:
   ```
   cargo run
   ```

## Security Notes

- The `.env` file containing sensitive credentials is gitignored
- Never commit your email credentials to version control
- Use app-specific passwords when possible instead of your main account password

