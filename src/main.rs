extern crate data_encoding;
extern crate ring;

use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, hmac, rand};
use data_encoding::HEXUPPER;

fn main() -> Result<(), Unspecified> {
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value);

    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);
    let raw_sig = hmac::sign(&key, "HEY".as_bytes());
    println!("{:?}", HEXUPPER.encode(raw_sig.as_ref()));

    hmac::verify(&key, "HEY".as_bytes(), &raw_sig.as_ref())?;

    Ok(())
}
