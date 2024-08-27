use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

fn main() {
    let email = Message::builder()
        .from("Support_osozn_mat <osoznanoe.materinstvo@gmail.com>".parse().unwrap())
        .to("ysfedoseev@yandex.ru".parse().unwrap())
        .subject("Test email from Rust")
        .body(String::from("This is a test email please do not reply!"))
        .unwrap();

    let creds = Credentials::new(
        String::from("osoznanoe.materinstvo@gmail.com"),
        String::from("aaor elyg bkzz quon")
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();
    
    match mailer.send(&email) {
        Ok(_) => {
            println!("Email send successfully")
        }
        Err(e) => {
            println!("Some err: {:?}", e)
        }
    }
}