extern crate data_encoding;
extern crate ring;

use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{ pbkdf2, rand};
use std::num::NonZeroU32;

fn main () -> Result<(), Unspecified> {
    let password= "NO WAY YOU CAN GUESS THIS BITCH!";
    const CREDENTIAN_LEN : usize = ring::digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(10).unwrap();
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; CREDENTIAN_LEN];

    rng.fill(&mut salt);

    let mut hashed_pass = [0u8; CREDENTIAN_LEN];
    pbkdf2::derive(pbkdf2::PBKDF2_HMAC_SHA512, n_iter, &salt, password.as_bytes(), &mut hashed_pass);

    println!("SALT: {}", HEXUPPER.encode(&salt));
    println!("PBKDF2 : {}", HEXUPPER.encode(&hashed_pass));

    //lets Verify
    let should_ok = pbkdf2::verify(pbkdf2::PBKDF2_HMAC_SHA512, n_iter, &salt, password.as_bytes(), &hashed_pass);
    assert!(should_ok.is_ok());

    let not_ok = pbkdf2::verify(pbkdf2::PBKDF2_HMAC_SHA512, n_iter, &salt, "SOME WRONG PASSWORD".as_bytes(), &hashed_pass);
    assert!(!not_ok.is_ok());
    Ok(())
}
