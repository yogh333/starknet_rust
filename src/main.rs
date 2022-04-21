use starknet::core::*;

fn main() {
    let message = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. 123");

    let hash = utils::starknet_keccak(message.as_bytes());

    println!("Message: {}", message);

    println!("Hash: {:?}", hash);

    let k = types::FieldElement::from_hex_be(
        "0x04a2c3811d30daef56d3025ab276ebe6bdfc70a8fc0bf22e9b8398af5954d8ef",
    )
    .unwrap();

    println!("Private key: {:?}", k);

    let signature = crypto::ecdsa_sign(&k, &hash).unwrap();

    println!("Signature: r = {}, s = {}", signature.r, signature.s);
}
