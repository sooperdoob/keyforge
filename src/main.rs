use bitcoin::{Address, Network, PrivateKey, PublicKey, Script};
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use rand::{thread_rng, Rng};

fn main() {
    // Generate random secret key.
    let secp = Secp256k1::new();
    let mut rng = thread_rng();
    let secret_key = SecretKey::from_slice(&mut rng.gen::<[u8; 32]>()).expect("Failed to generate secret key");

    // Convert the secret key to its corresponding private key structure.
    let private_key = PrivateKey {
        compressed: true,
        network: Network::Bitcoin,
        inner: secret_key,
    };

    // Create compressed PublicKey
    let compressed_public_key = PublicKey::from_private_key(&secp, &private_key);

    // Create uncompressed PublicKey
    let uncompressed_private_key = PrivateKey {
        compressed: false,
        network: Network::Bitcoin,
        inner: private_key.inner,
    };

    let uncompressed_public_key = PublicKey::from_private_key(&secp, &uncompressed_private_key);

    // Generate Legacy P2PKH addresses.
    let compressed_legacy_address = Address::p2pkh(&compressed_public_key, Network::Bitcoin);
    let uncompressed_legacy_address = Address::p2pkh(&uncompressed_public_key, Network::Bitcoin);

    // Generate SegWit Bech32 address.
    let compressed_segwit_bech32_address = Address::p2wpkh(&compressed_public_key, Network::Bitcoin).unwrap();

    // Generate SegWit Base58 address.
    let compressed_segwit_base58_address = Address::p2shwpkh(&compressed_public_key, Network::Bitcoin).unwrap();

    // Get the script pubkey from the SegWit address
    let script_pubkey = compressed_segwit_bech32_address.script_pubkey();

    // Manually construct the script code for P2WPKH
    let script_code = Script::new_p2pkh(&uncompressed_public_key.pubkey_hash());

    // Generate the P2WSH address
    // Needs debugged
    let p2wsh_address = Address::p2wsh(&script_code, Network::Bitcoin);

    // Print the P2WSH address
    println!("P2WSH Address (WIP): {}", p2wsh_address);

    // Print the script code
    println!("Script Code: {:?}", script_code);

    // Print the addresses
    println!("Compressed P2PKH: {}", compressed_legacy_address);
    println!("Uncompressed P2PKH: {}", uncompressed_legacy_address);
    println!("SegWit P2WPKH: {}", compressed_segwit_bech32_address);
    println!("SegWit P2SHWPKH: {}", compressed_segwit_base58_address);

    // Print the Wallet Import Format (WIF) of the private key
    println!("Compressed WIF: {}", private_key.to_wif());
    println!("Uncompressed WIF: {}", uncompressed_private_key.to_wif());
}
