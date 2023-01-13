
use std::env;
//use ecdsa_core::{signature::Verifier, Signature};
//use ecdsa::{};
//use k256::ecdsa::{SigningKey, signature::Signer, VerifyingKey,signature::Verifier, Signature};
//use p384::ecdsa::{SigningKey, signature::Signer, VerifyingKey,signature::Verifier, Signature};
use rand_core::OsRng; 

fn p384_fn() {
	use p384::ecdsa::{signature::Signer,signature::Verifier};
	let mut message = String::from("Hello world!");

	let args: Vec<String> = env::args().collect();
	if args.len() >1 {
		message = args[1].clone();
	}

	println!("Message: {}",message);

	let mut msg=message.as_bytes();

	// Signing
	let signing_key = p384::ecdsa::SigningKey::random(&mut OsRng); 
	let sk=signing_key.to_bytes();
	println!("\nSigning key: {:x?}",hex::encode(sk));

	let signature: p384::ecdsa::Signature = signing_key.sign(msg);
	println!("\nSignature key: {:x?}",hex::encode(signature));

	let verify_key = p384::ecdsa::VerifyingKey::from(&signing_key); 
	// Serialize with `::to_encoded_point()`
	let ep = verify_key.to_encoded_point(false);
	let vk= ep.as_bytes();
	println!("\nVerifying key: {:x?}",hex::encode(vk));



	let rtn=verify_key.verify(msg, &signature).is_ok();

	if rtn==true { println!("\nMessage '{0}' signature correct", message); }
	else { println!("\nMessage '{0}' signature incorrect",message);}

	msg="hello".as_bytes();

	let rtn=verify_key.verify(msg, &signature).is_ok();

	if rtn==true { println!("\nWith 'hello', message signature correct"); }
	else { println!("\nWith 'hello', message signature incorrect");}	
}

fn k256_fn() {
	use k256::ecdsa::{signature::Signer,signature::Verifier};
	let mut message = String::from("Hello world!");

	let args: Vec<String> = env::args().collect();
	if args.len() >1 {
		message = args[1].clone();
	}

	println!("Message: {}",message);

	let mut msg=message.as_bytes();

	// Signing
	let signing_key = k256::ecdsa::SigningKey::random(&mut OsRng); 
	let sk=signing_key.to_bytes();
	println!("\nSigning key: {:x?}",hex::encode(sk));

	let signature: k256::ecdsa::Signature = signing_key.sign(msg);
	println!("\nSignature key: {:x?}",hex::encode(signature));

	let verify_key = k256::ecdsa::VerifyingKey::from(&signing_key); 
	// Serialize with `::to_encoded_point()`
	let vk=verify_key.to_bytes();
	println!("\nVerifying key: {:x?}",hex::encode(vk));


	let rtn=verify_key.verify(msg, &signature).is_ok();

	if rtn==true { println!("\nMessage '{0}' signature correct", message); }
	else { println!("\nMessage '{0}' signature incorrect",message);}

	msg="hello".as_bytes();

	let rtn=verify_key.verify(msg, &signature).is_ok();

	if rtn==true { println!("\nWith 'hello', message signature correct"); }
	else { println!("\nWith 'hello', message signature incorrect");}	
}

fn main() {
	k256_fn();
	p384_fn();

}