use crate::utils::errors::ApiError;
use ad_buy_engine::constant::apis::private::API_URL_CONFIRM_EMAIL_INVITATION_ADD_USER_TO_ACCOUNT;
use ad_buy_engine::constant::apis::public::API_URL_CONFIRM_EMAIL_INVITATION;
use ad_buy_engine::constant::email::email_service_info::{
    EMAIL_SENDER_DOMAIN, EMAIL_SENDER_USERNAME, MAILGUN_API_KEY,
};
use ad_buy_engine::constant::server_info::{CAMPAIGN_SERVER_URL, ROOT_DOMAIN};
use ad_buy_engine::data::backend_models::invitation::Invitation;
use mailgun_rs::{EmailAddress, Mailgun, Message};
use std::error::Error;

// pub fn send_email_invite_another_user(invitation: &TeamInvitation) -> Result<(), ApiError> {
//     let recipient = invitation.email.as_str();
//     let recipient = EmailAddress::address(&recipient);
//     let message = Message {
//         to: vec![recipient],
//         subject: String::from("YOU ARE INVITED - RESPONSE REQUIRED"),
//         html: format!(
//             "Please click on the link below to complete your registration. <br/>
//          <a href=\"{}/{}/{}/{}\">
//          Verify Email</a>
//          your Invitation expires on <strong>{}</strong>",
//             CAMPAIGN_SERVER_URL,
//             "email",
//             "accept_invite",
//             invitation.id,
//             invitation
//                 .expires_at
//                 .format("%I:%M %p %A, %-d %B, %C%y")
//                 .to_string(),
//         ),
//         ..Default::default()
//     };
//
//     let client = Mailgun {
//         api_key: String::from(MAILGUN_API_KEY),
//         domain: String::from(EMAIL_SENDER_DOMAIN),
//         message,
//     };
//     let email = EmailAddress::name_address(
//         "no-reply",
//         format!("{}@{}", EMAIL_SENDER_USERNAME, EMAIL_SENDER_DOMAIN).as_str(),
//     );
//     let sent_email = client.send(&email);
//
//     // Note that we only print out the error response from email api
//     match sent_email {
//         Ok(res) => {
//             dbg!("Success: {}", res.message);
//             Ok(())
//         }
//         Err(err) => {
//             dbg!("Failed: {}", err.description());
//             Err(ApiError::InternalServerError(err.description().to_string()))
//         }
//     }
// }

pub fn send_invitation(invitation: &Invitation) -> Result<(), ApiError> {
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
        api_key: String::from(MAILGUN_API_KEY),
        domain: String::from(EMAIL_SENDER_DOMAIN),
        message,
    };
    let email = EmailAddress::name_address(
        EMAIL_SENDER_DOMAIN,
        format!("{}@{}", EMAIL_SENDER_USERNAME, EMAIL_SENDER_DOMAIN).as_str(),
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
