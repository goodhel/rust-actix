use actix_web::{web, HttpResponse, Error};
use serde::{Serialize, Deserialize};
use lettre::transport::smtp::{authentication::Credentials};
use lettre::{Message, SmtpTransport, Transport};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Email {
    nama: String,
    message: String,
    email: String,
    notif: String
}

pub async fn insert(data: web::Json<Email>) -> Result<HttpResponse, Error> {
        println!("data: {:?}", data);
        let to = format!("{} <{}>",data.nama,data.email);
        let message = &data.message;
        let email = Message::builder()
        .from("Virtual Rakornas <virtualrakornas@gmail.com>".parse().unwrap())
        .to(to.parse().unwrap())
        .subject(&data.notif)
        .body(message.to_string())
        .unwrap();

        // let creds = Credentials::new("virtualiai2020@gmail.com".to_string(), "Virtualiai-2020".to_string());
        let creds = Credentials::new("virtualrakornas@gmail.com".to_string(), "Virtualrakornas-2021".to_string());
        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => {
                println!("Email sent successfully!");
                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "Status": "OK",
                    "Info": "Send Email Berhasil"
                })))
            },
            Err(e) => {
                panic!("Could not send email: {:?}", e);
            },
        }

        // match _email {
        //     Ok(_data) => {
        //         Ok(HttpResponse::Ok().json(serde_json::json!({
        //             "Status": "OK",
        //             "Info": "Send Email Berhasil"
        //         })))
        //     }
        //     _ => Ok(HttpResponse::Ok().json(serde_json::json!({
        //         "Status": "OK",
        //         "Info": "Send Email Gagal"
        //     })))
        // }
}