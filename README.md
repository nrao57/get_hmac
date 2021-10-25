# get_hmac

## Overview
`get_hmac` is a utility CLI program written in Rust to get the HMAC SHA256 signature using a Secret Key and Message

## Building the Rust Binary
Run the following command `cargo build --release`

After, there should be a binary located in `./target/release` named `get_hmac`


## To Get the HMAC SHA256 Signature of a JSON file

Run the following command
```
./target/release/get_hmac SECRET_KEY PATH_TO_JSON_FILE
```

## Validation
To check the binary is built correctly, run the command
```
 ./target/release/get_hmac "testKey" "./payloads/testPayload.json"
```

The command should result in
```
8jvepp6AoAofwBNMx2rbuxjaEf/monBBJLLb301FOR8=
```
