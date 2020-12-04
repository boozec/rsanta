extern crate clap;
extern crate lettre;

use clap::{App, Arg};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs;

fn my_gift_to(players: &mut Vec<String>) -> Option<&String> {
    let mut rng = rand::thread_rng();
    players.shuffle(&mut rng);

    Some(&players[0])
}

fn main() {
    let matches = App::new("rsanta")
        .version("0.1.0")
        .author("Santo Cariotti <santo@dcariotti.me>")
        .about("Make secret santa with your friends")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Input file for players")
                .takes_value(true),
        )
        .get_matches();

    let config_file = matches
        .value_of("file")
        .expect("You must specify a config file");
    let contents = fs::read_to_string(config_file)
        .expect("Error occurs reading this file")
        .to_string();

    let config_email;
    let config_password;
    let host;
    match env::var("EMAIL") {
        Ok(v) => config_email = v.to_string(),
        Err(e) => panic!("Must provide EMAIL: {}", e),
    }
    match env::var("PASSWORD") {
        Ok(v) => config_password = v.to_string(),
        Err(e) => panic!("Must provide PASSWORD: {}", e),
    }
    match env::var("HOST") {
        Ok(v) => host = v,
        Err(e) => panic!("Must provide HOST: {}", e),
    }

    let mut members_data: Vec<&str> = contents.split(">\n").collect();

    // ignore the last element, cause it's empty
    members_data.pop();

    let mut rng = rand::thread_rng();
    members_data.shuffle(&mut rng);

    let mut members = HashMap::new();
    for member in members_data {
        let details: Vec<&str> = member.split("<").collect();

        // <email> : <name>
        members.insert(details[1], details[0]);
    }

    let mut done: Vec<String> = Vec::new();
    let mut emails: Vec<String> = Vec::new();
    for (email, _) in &members {
        emails.push(email.to_string());
    }

    let creds = Credentials::new(config_email, config_password);

    let mailer = SmtpTransport::relay(&host)
        .unwrap()
        .credentials(creds)
        .build();

    for (email, name) in members.into_iter() {
        loop {
            match my_gift_to(&mut emails) {
                Some(gift_to) => {
                    if gift_to.to_string() != email.to_string()
                        && !done.iter().any(|v| v == gift_to)
                    {
                        let mail = Message::builder()
                            .from("Santo Cariotti <santo@dcariotti.me>".parse().unwrap())
                            .to(format!("{} <{}>", name, email).parse().unwrap())
                            .subject("Secret Santa!")
                            .body(format!("You're the Secret Santa of:\n{}", name))
                            .unwrap();
                        match mailer.send(&mail) {
                            Ok(_) => println!("Email sent successfully to {}!", email),
                            Err(e) => panic!("Could not send email: {:?}", e),
                        }
                        done.push(gift_to.to_string());
                        break;
                    }
                }
                None => break,
            }
        }
    }
}
