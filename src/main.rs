use bitcoin::{Address, Network};
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey as BitcoinPublicKey};
use rand::thread_rng;
use rand::Rng;

fn main() {
    // Generate random secret key.
    let secp = Secp256k1::new();
    let mut rng = thread_rng();
    let secret_key = SecretKey::from_slice(&mut rng.gen::<[u8; 32]>()).expect("Failed to generate secret key");

    // Convert the secret key to its corresponding private key structure.
    let private_key = bitcoin::PrivateKey {
        compressed: true,
        network: Network::Bitcoin,
        inner: secret_key,
    };

    // Derive public key from the secret key.
    let public_key = BitcoinPublicKey::from_secret_key(&secp, &private_key.inner);

    // Serialize the public key.
    let pubkey_bytes = public_key.serialize();

    // Convert the byte slice into a bitcoin::PublicKey instance.
    let pubkey = bitcoin::PublicKey::from_slice(&pubkey_bytes).expect("Failed to create PublicKey");

    // Generate Legacy P2PKH address.
    let legacy_address = Address::p2pkh(&pubkey, Network::Bitcoin);

    // Generate SegWit Bech32 address.
    let segwit_bech32_address = Address::p2wpkh(&pubkey, Network::Bitcoin).unwrap();

    // Generate SegWit Base58 address.
    let segwit_base58_address = Address::p2shwpkh(&pubkey, Network::Bitcoin).unwrap();

    // Convert the private key to WIF format.
    let wif_key = private_key.to_wif();

    println!("Legacy P2PKH: {}", legacy_address);
    println!("SegWit P2WPKH: {}", segwit_bech32_address);
    println!("SegWit P2SH: {}", segwit_base58_address);
    println!("WIF key: {}", wif_key);
}
