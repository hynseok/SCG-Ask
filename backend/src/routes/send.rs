use std::env;
use std::time::Duration;

use actix_web::{web, HttpResponse};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
    content: String,
}

pub async fn send(form: web::Json<FormData>) -> HttpResponse {
    let body = format!(
        "회신 이메일: {}\n\n문의 내용:\n{}",
        form.email, form.content
    );

    let email = Message::builder()
        .from("<scg@scg.skku.ac.kr>".parse().unwrap())
        .to("<scg@scg.skku.ac.kr>".parse().unwrap())
        .subject(format!("[SCG Ask] {}님의 문의입니다.", form.name))
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .unwrap();

    let creds = Credentials::new(
        env::var("SMTP_USERNAME")
            .expect("SMTP_USERNAME must be set.")
            .to_owned(),
        env::var("SMTP_PASSWORD")
            .expect("SMTP_PASSWORD must be set.")
            .to_owned(),
    );

    let mailer = SmtpTransport::starttls_relay("mail.scg.skku.ac.kr")
        .unwrap()
        .port(587) 
        .credentials(creds)
        .timeout(Some(Duration::from_secs(15)))
        .build();

    match mailer.send(&email) {
        Ok(_) => HttpResponse::Ok().body("Email sent successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to send email: {}", e)),
    }
}
