use rsa::{PublicKey, PaddingScheme};

fn rsa_encrypt(pub_key: &str, password: &str) -> String {
    let decoded_pub_key = base64::decode(pub_key).unwrap();
    let rsa_pub_key = rsa::RSAPublicKey::from_pkcs8(&decoded_pub_key).unwrap();
    let rng = &mut rand::thread_rng();
    let encrypt_pass = rsa_pub_key.encrypt(rng, PaddingScheme::new_pkcs1v15_encrypt(), password.as_bytes()).unwrap();
    base64::encode(encrypt_pass)
}

fn main() {
    let pukey = "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDSL1TeOBHsuj48Ep3Uh5SIanpSwFcavsGTz71bq+P2oIoDqNKZzeY6zHlFpHqvEesjjSJE5ZnbIIWQCJyfD+6x8w5+MhCm0G394rvrvf5jWKrc919GzViymo73c9M8+r+A9HU94+dAorpCzVRm+N/1XOrfoNIrLRDPE43h4OtDJQIDAQAB";
    println!("{}", rsa_encrypt(pukey, "df723820"));
}
