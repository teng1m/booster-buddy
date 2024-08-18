use rouille::Response;

pub enum ParseResult {
    //       token    name   exp
    Success((String, String, u64)),
    Error(String),
    NoOp,
}

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

    // Vec<(String, String)>
    //  |     |        |-- Value
    //  |     |----------- Key
    //  |----------------- Tokens

    let mut result = ("".to_string(), "".to_string(), 0);
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
            "access_token" => result.0 = token.1,
            "nickname" => result.1 = token.1,
            "expires_at" => result.2 = token.1.parse().unwrap_or_default(),
            _ => println!("Found {}: {}", token.0, token.1),
        }
    }
    println!("payload has been ITERATED");

    return ParseResult::Success(result);
}

pub fn start_response_server() {
    rouille::start_server("0.0.0.0:5000", move |request| {
        match parse_http_payload(request) {
            ParseResult::Success(result) => {
                // log authentication
                Response::html(format!(include_str!("../200.html"), result.1, result.2))
            }
            ParseResult::Error(err) => Response::html(format!(include_str!("../404.html"), err)),
            ParseResult::NoOp => Response::empty_204(),
        }
    });
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_html_format() {
        println!(
            "{}",
            format!(include_str!("../200.html"), "SanguineParadise", 21403153)
        );
    }
}
