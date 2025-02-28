use lettre::{
    Address, Message, SmtpTransport, Transport,
    message::{Mailbox, SinglePart, header},
    transport::smtp::authentication::Credentials,
};
use std::io::{Error, ErrorKind};

/// Main function that sends emails to multiple recipients using Gmail SMTP.
///
/// # Required Environment Variables
///
/// * `SENDER` - Gmail address to send emails from such as "sender@gmail.com"
/// * `APP_PASSWORD` - Gmail App Password for SMTP authentication
///   Generate at <https://myaccount.google.com/security> under "2-Step Verification" > "App passwords"
///
/// # Required Files
///
/// * `recipients.json` - JSON array of recipient information:
///   ```json
///   [
///     {
///       // email address and some information to be sent to the recipient
///     },
///     ...
///   ]
///   ```
///

fn main() -> std::io::Result<()> {
    let username = std::env::var("SENDER")
        .map_err(|e| Error::new(ErrorKind::NotFound, format!("env:SENDER not found: {}", e)))?;
    // Gmail App Password is required for SMTP authentication
    // You can generate one at https://myaccount.google.com/security
    // Under "2-Step Verification" > "App passwords"
    // Select "Mail" and your device, then use the generated 16-character password
    let password = std::env::var("APP_PASSWORD").map_err(|e| {
        Error::new(
            ErrorKind::NotFound,
            format!("env:APP_PASSWORD not found: {}", e),
        )
    })?;
    let targets = include_str!("../recipients.json");
    let receivers: Vec<serde_json::Value> = serde_json::from_str::<serde_json::Value>(&targets)
        .map_err(|e| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Failed to parse recipients.json: {}", e),
            )
        })?
        .as_array()
        .ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                "recipients.json must be a JSON array",
            )
        })?
        .into_iter()
        .map(|value| value.to_owned())
        .collect::<Vec<_>>();
    let sender = SmtpTransport::starttls_relay("smtp.gmail.com")
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to create SMTP relay: {}", e),
            )
        })?
        .credentials(Credentials::new(username.to_string(), password.to_string()))
        .build();
    sender.test_connection().map_err(|e| {
        Error::new(
            ErrorKind::ConnectionRefused,
            format!("Failed to connect to SMTP server: {}", e),
        )
    })?;

    let mut errors = Vec::new();

    for (i, value) in receivers.iter().enumerate() {
        println!("Sending email: {}/{}", i + 1, receivers.len());
        let receiver = receiver_email_from_value(value);
        let subject = "This is a test email";

        let receiver_address = match receiver.parse::<Address>() {
            Ok(addr) => addr,
            Err(e) => {
                errors.push(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid recipient email address: {}", e),
                ));
                continue;
            }
        };

        let sender_address = match username.parse::<Address>() {
            Ok(addr) => addr,
            Err(e) => {
                errors.push(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid sender email address: {}", e),
                ));
                continue;
            }
        };

        let email = match Message::builder()
            .to(Mailbox::new(None, receiver_address))
            .from(Mailbox::new(None, sender_address))
            .subject(subject)
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_PLAIN)
                    .body(body_from_json_value(&value)),
            ) {
            Ok(msg) => msg,
            Err(e) => {
                errors.push(Error::new(
                    ErrorKind::Other,
                    format!("Failed to build email message: {}", e),
                ));
                continue;
            }
        };

        if let Err(e) = sender.send(&email) {
            println!("Unable to send email {}/{}: {}", i + 1, receivers.len(), e);
            errors.push(Error::new(
                ErrorKind::Other,
                format!("Failed to send email: {}", e),
            ));
        }
    }

    if !errors.is_empty() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Encountered errors: {:?}", errors),
        ));
    }

    Ok(())
}

/// Implement your own logic to obtain email address from serde_json::Value
///
/// # Arguments
///
/// * `value` - A JSON value containing email address information
///
/// # Returns
///
/// The email address as a String
///
/// # Example
///
/// ```no_run
/// // For JSON like: {"email": "example@test.com", ...}
/// // You might implement:
/// fn receiver_email_from_value(value: &serde_json::Value) -> String {
///     value.as_object().unwrap().get("email").unwrap().as_str().unwrap().to_string()
/// }
/// ```

fn receiver_email_from_value(_: &serde_json::Value) -> String {
    todo!()
}

/// Implement your own logic to obtain email body from serde_json::Value
///
/// # Arguments
///
/// * `value` - A JSON value containing email body information
///
/// # Returns
///
/// The email body as a String
///
/// # Example
///
/// ```no_run
/// // For JSON like: {"body": "Hello world", ...}
/// // You might implement:
/// fn body_from_json_value(value: &serde_json::Value) -> String {
///     value.as_object().unwrap().get("body").unwrap().as_str().unwrap().to_string()
/// }
/// ```

fn body_from_json_value(_: &serde_json::Value) -> String {
    todo!()
}
