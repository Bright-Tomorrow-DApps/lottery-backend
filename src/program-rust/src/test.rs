extern crate easy_hasher;
use easy_hasher::easy_hasher::*;

fn main() {
    let string = "example string".to_string();
    let hash = sha256(&string);
    let string_hash = hash.to_hex_string();

    assert_eq!(
        string_hash,
        "aedfb92b3053a21a114f4f301a02a3c6ad5dff504d124dc2cee6117623eec706"
    );
    println!("SHA256({}) = {}", string, string_hash);
}
