use crate::utils::errors::ApiError;
use ad_buy_engine::constant::apis::public::API_URL_CONFIRM_EMAIL_INVITATION;
use ad_buy_engine::constant::email::email_service_info::{
    EMAIL_SENDER_DOMAIN, EMAIL_SENDER_DOMAIN_ENV, EMAIL_SENDER_USERNAME, EMAIL_SENDER_USERNAME_ENV,
    MAILGUN_API_KEY_ENV,
};
use ad_buy_engine::constant::server_info::ROOT_DOMAIN;
use ad_buy_engine::data::backend_models::invitation::Invitation;
use mailgun_rs::{EmailAddress, Mailgun, Message};
use std::env;
use std::error::Error;

pub fn send_invitation(invitation: &Invitation) -> Result<(), ApiError> {
    let mailgun_api_key = env::var(MAILGUN_API_KEY_ENV).map_err(|_| {
        ApiError::InternalServerError(format!("{} must be set", MAILGUN_API_KEY_ENV))
    })?;
    let email_sender_domain =
        env::var(EMAIL_SENDER_DOMAIN_ENV).unwrap_or_else(|_| EMAIL_SENDER_DOMAIN.to_string());
    let email_sender_username =
        env::var(EMAIL_SENDER_USERNAME_ENV).unwrap_or_else(|_| EMAIL_SENDER_USERNAME.to_string());

    let recipient = invitation.email.as_str();
    let recipient = EmailAddress::address(&recipient);
    let message = Message {
        to: vec![recipient],
        subject: String::from("RESPONSE REQUIRED - CONFIRM EMAIL"),
        html: format!(
            "Please click on the link below to complete your registration. <br/>
         <a href=\"{}{}/{}\">
         Verify Email</a>
         your Invitation expires on <strong>{}</strong>",
            ROOT_DOMAIN,
            API_URL_CONFIRM_EMAIL_INVITATION,
            invitation.id,
            invitation
                .expires_at
                .format("%I:%M %p %A, %-d %B, %C%y")
                .to_string(),
        ),
        ..Default::default()
    };

    let client = Mailgun {
        api_key: mailgun_api_key,
        domain: email_sender_domain.clone(),
        message,
    };
    let email = EmailAddress::name_address(
        email_sender_domain.as_str(),
        format!("{}@{}", email_sender_username, email_sender_domain).as_str(),
    );

    let sent_email = client.send(&email);

    match sent_email {
        Ok(res) => {
            dbg!("Success: {}", res.message);
            Ok(())
        }
        Err(err) => {
            dbg!("Failed: {}", err.description());
            Err(ApiError::InternalServerError(err.description().to_string()))
        }
    }
}
