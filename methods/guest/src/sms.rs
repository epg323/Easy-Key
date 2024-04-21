use dotenv::dotenv;
use http::{request, Request};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};

#[derive(Serialize, Deserialize)]
struct SMSResponse {}

#[derive(Serialize, Deserialize)]
struct SubresourceUris {}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {}

fn handle_error(body: String) {}

fn handle_success(body: String) {}

pub fn send_sms(to: String) -> std::io::Result<()> {
    println!("hello");
    // dotenv().expect("Error reading .env file");

    let listener = TcpListener::bind("127.0.0.1:80")?;

    println!("made it");

    for stream in listener.incoming() {
        if stream.is_ok() {
            println!("listening {}", stream.unwrap().local_addr().unwrap().ip());
        } else {
            println!("dud ");
        }
    }
    println!("Listening on http://{}", "hi");

    // let account_sid = env::var("TWILIO_ACCOUNT_SID").expect("Error reading Twilio Account SID");
    // let api_key_sid = env::var("TWILIO_API_KEY_SID").expect("Error reading Twilio API key");
    // let api_key_secret = env::var("TWILIO_API_KEY_SECRET").expect("Error reading Twilio API SID");
    // let from = env::var("TWILIO_PHONE_NUMBER").expect("Error reading Twilio Phone Number");
    // let auth_token =
    //     env::var("TWILIO_AUTH_TOKEN").expect("Twilio Auth Token could not be retrieved.");

    // let request_params = [
    //     ("To", &to),
    //     ("From", &from),
    //     // ("Body", &sms_body.to_string()),
    // ];

    // let sms_body = "3247983 is your authentication code for EasyKey";
    // let request_url =
    //     format!("https://api.twilio.com/2010-04-01/Accounts/{account_sid}/Messages.json");
    // let client = Request::builder();
    // let request = client.method("POST").uri(request_url).body(sms_body);

    Ok(())
    // let response = send(request.unwrap());

    // // .post(request_url)
    // // .basic_auth(account_sid, Some(auth_token))
    // // .form(&request_params)
    // // .send()?;

    // let status = response.unwrap().;
    // let body = match response.text() {
    //     Ok(result) => result,
    //     Err(error) => panic!(
    //         "Problem extracting the JSON body content. Reason: {:?}",
    //         error
    //     ),
    // };

    // match status {
    //     StatusCode::BAD_REQUEST => handle_error(body),
    //     StatusCode::OK => handle_success(body),
    //     _ => println!("Received status code: {}", status),
    // }

    // println!("{:?}", response.text());
}
