use rouille::Response;
use std::{collections::HashMap, fs};

pub enum ParseResult {
    Success(String),
    Error(String),
    NoOp,
}

pub struct AuthServer {
    discord_wg_id_associations: HashMap<String, String>,
}

impl AuthServer {
    pub fn parse_http_payload(payload: &rouille::Request) -> ParseResult {
        if payload.raw_query_string().is_empty() {
            return ParseResult::NoOp;
        }

        println!("Doing shit with payload {}", payload.raw_query_string());
        // split the payload into chunks, e.g:
        // &status=ok&access_token=db52654da9792a6898cccb9091a271dd4f93496d&nickname=SanguineParadise&account_id=1052310511&expires_at=1725094097
        // becomes
        // ["status=ok", "access_token=db52654da9792a6898cccb9091a271dd4f93496d", "nickname=SanguineParadise", "account_id=1052310511", "expires_at=1725094097"]
        let payload_split: Vec<(String, String)> = payload
            .raw_query_string()
            .split('&')
            .filter(|token| !token.is_empty())
            .map(|token| {
                let split_token: Vec<&str> = token.split('=').collect();
                let mut result = ("".to_string(), "".to_string());
                if split_token.len() > 0 {
                    result.0 = split_token[0].to_string()
                }
                if split_token.len() > 1 {
                    result.1 = split_token[1].to_string()
                }
                // anything else gets thrown away!! (not possible)
                result
            })
            .collect::<Vec<(String, String)>>();
        println!("payload has been SPLIT: {:?}", payload_split);

        // <Vec<(String, String)
        //   |     |        |-- Value
        //   |     |----------- Key
        //   |----------------- Tokens

        for token in payload_split {
            match token.0.as_str() {
                "status" => {
                    if token.1 == "error".to_string() {
                        return ParseResult::Error("What went wrong lmao".to_string());
                    // TODO: enrich error reporting to inject in HTML
                    } else {
                        println!("Received ok status from WG API");
                    }
                }
                _ => println!("Found {}: {}", token.0, token.1),
            }
        }
        println!("payload has been ITERATED");

        return ParseResult::Success("Player ID".to_string()); // TODO: Insert Player ID
    }

    pub async fn listen_for_auth_responses(&self) {
        rouille::start_server(
            "0.0.0.0:5000",
            move |request| match Self::parse_http_payload(request) {
                ParseResult::Success(name) => {
                    // log authentication
                    Response::text(format!("Thanks, {}! You're authenticated.", name))
                }
                ParseResult::Error(err) => Response::text(format!("Error authenticating: {}", err)),
                ParseResult::NoOp => Response::empty_204(),
            },
        );
    }

    pub fn start_server(&self) {
        // this is broken
        //tokio::spawn(self.listen_for_auth_responses());
        // do I need to have 3 threads (main, listener, authentication handler)? the main thread is the "discord handler", so it should not be responsible for handling player ID associations.
    }
}
