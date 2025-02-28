use lettre::{
    message::{header, Mailbox, SinglePart},
    transport::smtp::authentication::Credentials,
    Address, Message, SmtpTransport, Transport,
};

/// Main function that sends emails to multiple recipients using Gmail SMTP.
///
/// # Required Environment Variables
///
/// * `SENDER` - Gmail address to send emails from such as "sender@gmail.com"
/// * `APP_PASSWORD` - Gmail App Password for SMTP authentication
///   Generate at https://myaccount.google.com/security under "2-Step Verification" > "App passwords"
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

fn main() {
    let username = std::env::var("SENDER").expect("env:SENDER not found");
    // Gmail App Password is required for SMTP authentication
    // You can generate one at https://myaccount.google.com/security
    // Under "2-Step Verification" > "App passwords"
    // Select "Mail" and your device, then use the generated 16-character password
    let password = std::env::var("APP_PASSWORD").expect("env:APP_PASSWORD not found");
    let targets = include_str!("../recipients.json");
    let receivers: Vec<serde_json::Value> = serde_json::from_str::<serde_json::Value>(&targets)
        .unwrap()
        .as_array()
        .unwrap()
        .into_iter()
        .map(|value| value.to_owned())
        .collect::<Vec<_>>();
    let sender = SmtpTransport::starttls_relay("smtp.gmail.com")
        .unwrap()
        .credentials(Credentials::new(username.to_string(), password.to_string()))
        .build();
    assert!(sender.test_connection().expect("OK"));

    for (i, value) in receivers.iter().enumerate() {
        println!("Sending email: {}/{}", i + 1, receivers.len());
        let receiver = receiver_email_from_value(value);
        let subject = "This is a test email";
        let email = Message::builder()
            .to(Mailbox::new(None, receiver.parse::<Address>().unwrap()))
            .from(Mailbox::new(None, username.parse::<Address>().unwrap()))
            .subject(subject)
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_PLAIN)
                    .body(body_from_json_value(&value)),
            )
            .unwrap();
        let result = sender.send(&email);

        if let Err(e) = result {
            println!("Unable to send email {}/{}: {}", i + 1, receivers.len(), e);
        };
    }
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
/// ```
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
/// ```
/// // For JSON like: {"body": "Hello world", ...}
/// // You might implement:
/// fn body_from_json_value(value: &serde_json::Value) -> String {
///     value.as_object().unwrap().get("body").unwrap().as_str().unwrap().to_string()
/// }
/// ```

fn body_from_json_value(_: &serde_json::Value) -> String {
    todo!()
}
