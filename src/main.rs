use std::env;
use std::fs;
use ring::{hmac};
use data_encoding::BASE64;

fn main() {
    // Get CLI Args
    let given_args: Vec<String> = env::args().collect();
    let secret_key = &given_args[1];
    let json_path = &given_args[2];

    // Convert JSON to String
    let payload:String = fs::read_to_string(&json_path).expect("failed to parse file").parse().expect("failed to read file");

    let signed_key = hmac::Key::new(hmac::HMAC_SHA256, &secret_key.as_bytes());
    let signature = hmac::sign(&signed_key, payload.as_bytes());
    let b64_encoded_sig = BASE64.encode(signature.as_ref());

    // To get underlying array use `into_bytes` method, but be careful, since
    // incorrect use of the code value may permit timing attacks which defeat
    // the security provided by the `Output`
    println!("{}", b64_encoded_sig);
}
