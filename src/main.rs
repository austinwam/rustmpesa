extern crate base64;

use reqwest::header::CONTENT_TYPE;

use base64::encode;


#[tokio::main]
async fn main() {
    let geta = getaccesstoken();
    geta.await;
}

async fn authmpesa() -> String {
    let consumer_key = "consumer_key:";
    let consumer_secret = "consumer_secret";
    let authurl = "https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials";
    let s1 = String::from(consumer_key);
    let s2 = String::from(consumer_secret);
    let s3 = concat(s1, s2);
    println!("{}", s3);
    let result = encode(s3);
    println!("{}", result);

    result
}

fn concat(s1: String, s2: String) -> String {
    s1 + &s2
}

async fn getaccesstoken() -> Result<(), reqwest::Error> {

    let res = authmpesa().await;
    println!("{}", res);
    let auth = "testauth";
    let authurl = "https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials";

    let client = reqwest::Client::new();
    let response = client
        .get(authurl)
        // .header(AUTHORIZATION, "Bearer {}")
        .header("Authorization", "Bearer ".to_owned() + &auth)
        .header(CONTENT_TYPE, "application/json") 
        .send()
        .await
        .unwrap();

    println!("body = {:?}", response);
    Ok(())
}



 