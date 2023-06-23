//! Send emails using the SMTP protocol.

use crate::config;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rocket::fairing::{Fairing, Info, Kind};

/// Struct for sending emails.
pub struct Mailer {
    client: SmtpTransport,
}

impl Mailer {
    /// Create a new mailer.
    pub fn new() -> Self {
        let client = SmtpTransport::relay(&config::SMTP_SERVER)
            .expect("Failed to create Mailer client")
            .credentials(Credentials::new(
                config::SMTP_USERNAME.to_string(),
                config::SMTP_PASSWORD.to_string(),
            ))
            .build();

        Self { client }
    }

    /// Send an email.
    pub fn send(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from(config::SMTP_USERNAME.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body.to_string())?;

        self.client.send(&email)?;
        Ok(())
    }
}

impl Fairing for Mailer {
    fn info(&self) -> Info {
        Info {
            name: "Mailer",
            kind: Kind::Request,
        }
    }
}
