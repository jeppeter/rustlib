//! AES256 CBC、CTR mode encrypt decrypt demo
extern crate crypto;
use std::str;
//use crypto::{buffer,aes,blockmodes,aessafe};
use crypto::buffer::{ReadBuffer,WriteBuffer};
use crypto::symmetriccipher::{BlockEncryptor,BlockDecryptor};
//use crypto;
use aes;
use aes::cipher::KeyIvInit;
use aes::cipher::AsyncStreamCipher;
use aes::cipher::BlockEncryptMut;
use aes::cipher::BlockDecryptMut;
use cbc;
use cfb_mode;
//use rand::{Rng};
//use rand::rngs::{OsRng};

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

use std::error::Error;

extargs_error_class!{AesLibError}

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

pub fn aes256_cbc_pure_encrypt(data: &[u8],key: &[u8], iv: &[u8])->Result<Vec<u8>,Box<dyn Error>>{
    let mut retdata  :Vec<u8> = Vec::new();
    let clen :usize;
    for i in 0..data.len() {
        retdata.push(data[i]);
    }
    if (data.len() % 16) != 0 {
        clen = (data.len() + 15 ) / 16;
    } else {
        clen = data.len() + 16;
    }

    while retdata.len() < clen {
        retdata.push(0);
    }
    let eo = Aes256CbcEnc::new(key.into(),iv.into()).encrypt_padded_mut::<aes::cipher::block_padding::Pkcs7>(&mut retdata,data.len());
    if eo.is_err() {
        let e = eo.err().unwrap();
        extargs_new_error!{AesLibError,"encrypt error {}", e}
    }
    let nd = eo.unwrap();
    Ok(nd.to_vec())
}

pub fn aes256_cbc_pure_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
    let mut retdata :Vec<u8> = Vec::new();
    for i in 0..encrypted_data.len() {
        retdata.push(encrypted_data[i]);
    }
    if (retdata.len() % 16) != 0 {
        extargs_new_error!{AesLibError,"not valid len [{}] % 16 != 0", retdata.len()}
    }

    let eo = Aes256CbcDec::new(key.into(),iv.into()).decrypt_padded_mut::<aes::cipher::block_padding::Pkcs7>(&mut retdata);
    if eo.is_err() {
        let e = eo.err().unwrap();
        extargs_new_error!{AesLibError,"decrypt error {}", e}
    }
    let nd = eo.unwrap();
    Ok(nd.to_vec())
}

// Encrypt a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
pub fn aes256_cbc_encrypt(data: &[u8],key: &[u8], iv: &[u8])->Result<Vec<u8>,Box<dyn Error>>{
    let mut encryptor=crypto::aes::cbc_encryptor(
        crypto::aes::KeySize::KeySize256,
        key,
        iv,
        crypto::blockmodes::PkcsPadding);

    let mut final_result=Vec::<u8>::new();
    let mut read_buffer=crypto::buffer::RefReadBuffer::new(data);
    let mut buffer=[0;4096];
    let mut write_buffer=crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop{
        let ro=encryptor.encrypt(&mut read_buffer,&mut write_buffer,true);
        if ro.is_err() {
            let e = ro.err().unwrap();
            extargs_new_error!{AesLibError,"encrypt error [{:?}]",e}
        }
        let result = ro.unwrap();

        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            crypto::buffer::BufferResult::BufferUnderflow=>break,
            crypto::buffer::BufferResult::BufferOverflow=>{},
        }
    }

    Ok(final_result)
}

// Decrypts a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
pub fn aes256_cbc_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
    let mut decryptor = crypto::aes::cbc_decryptor(
        crypto::aes::KeySize::KeySize256,
        key,
        iv,
        crypto::blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let ro = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);
        if ro.is_err() {
            let e = ro.err().unwrap();
            extargs_new_error!{AesLibError,"decrypt error [{:?}]",e}
        }
        let result = ro.unwrap();
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            crypto::buffer::BufferResult::BufferUnderflow => break,
            crypto::buffer::BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

pub fn aes128_encrypt(data :&[u8],key :&[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
    if key.len() != 16 {
        extargs_new_error!{AesLibError,"key [{}] != 16", key.len()}
    }
    if (data.len() % 16 ) != 0 {
        extargs_new_error!{AesLibError,"data [{}] % 16 != 0", data.len()}
    }

    let aes_enc = crypto::aessafe::AesSafe128Encryptor::new(key);
    let mut retdata :Vec<u8> = Vec::new();
    for _ in 0..data.len() {
        retdata.push(0);
    }
    for _ in 0..8 {
        retdata.push(0);
    }
    aes_enc.encrypt_block(&data[..], &mut retdata[..]);
    Ok(retdata)
}

pub fn aes128_decrypt(encdata :&[u8],key :&[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
    if key.len() != 16 {
        extargs_new_error!{AesLibError,"key [{}] != 16", key.len()}
    }
    if (encdata.len() % 16 ) != 8 {
        extargs_new_error!{AesLibError,"data [{}] % 16 != 8", encdata.len()}
    }

    let aes_dec = crypto::aessafe::AesSafe128Decryptor::new(key);
    let mut retdata :Vec<u8> = Vec::new();
    for _ in 0..(encdata.len()-8) {
        retdata.push(0);
    }
    aes_dec.decrypt_block(&encdata[..], &mut retdata[..]);
    Ok(retdata)
}

pub fn aes192_encrypt(data :&[u8],key :&[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
    if key.len() != 24 {
        extargs_new_error!{AesLibError,"key [{}] != 16", key.len()}
    }
    if (data.len() % 16 ) != 0 {
        extargs_new_error!{AesLibError,"data [{}] % 16 != 0", data.len()}
    }

    let aes_enc = crypto::aessafe::AesSafe192Encryptor::new(key);
    let mut retdata :Vec<u8> = Vec::new();
    for _ in 0..data.len() {
        retdata.push(0);
    }
    for _ in 0..8 {
        retdata.push(0);
    }
    aes_enc.encrypt_block(&data[..], &mut retdata[..]);
    Ok(retdata)
}

pub fn aes192_decrypt(encdata :&[u8],key :&[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
    if key.len() != 24 {
        extargs_new_error!{AesLibError,"key [{}] != 16", key.len()}
    }
    if (encdata.len() % 16 ) != 8 {
        extargs_new_error!{AesLibError,"data [{}] % 16 != 0", encdata.len()}
    }

    let aes_dec = crypto::aessafe::AesSafe192Decryptor::new(key);
    let mut retdata :Vec<u8> = Vec::new();
    for _ in 0..(encdata.len()-8) {
        retdata.push(0);
    }
    aes_dec.decrypt_block(&encdata[..], &mut retdata[..]);
    Ok(retdata)
}

pub fn aes256_encrypt(data :&[u8],key :&[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
    if key.len() != 32 {
        extargs_new_error!{AesLibError,"key [{}] != 16", key.len()}
    }
    if (data.len() % 16 ) != 0 {
        extargs_new_error!{AesLibError,"data [{}] % 16 != 0", data.len()}
    }

    let aes_enc = crypto::aessafe::AesSafe256Encryptor::new(key);
    let mut retdata :Vec<u8> = Vec::new();
    for _ in 0..data.len() {
        retdata.push(0);
    }
    for _ in 0..8 {
        retdata.push(0);
    }
    aes_enc.encrypt_block(&data[..], &mut retdata[..]);
    Ok(retdata)
}

pub fn aes256_decrypt(encdata :&[u8],key :&[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
    if key.len() != 32 {
        extargs_new_error!{AesLibError,"key [{}] != 16", key.len()}
    }
    if (encdata.len() % 16 ) != 8 {
        extargs_new_error!{AesLibError,"data [{}] % 16 != 0", encdata.len()}
    }

    let aes_dec = crypto::aessafe::AesSafe256Decryptor::new(key);
    let mut retdata :Vec<u8> = Vec::new();
    for _ in 0..(encdata.len()-8) {
        retdata.push(0);
    }
    aes_dec.decrypt_block(&encdata[..], &mut retdata[..]);
    Ok(retdata)
}

pub type Aes256CfbEnc = cfb_mode::Encryptor<aes::Aes256>;
pub type Aes256CfbDec = cfb_mode::Decryptor<aes::Aes256>;


pub fn aes256_cfb_encrypt(data :&[u8],key :&[u8], iv :&[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
    let mut retdata :Vec<u8> = data.to_vec();
    Aes256CfbEnc::new(key.into(),iv.into()).encrypt(&mut retdata);
    Ok(retdata)
}

pub fn aes256_cfb_decrypt(encdata :&[u8],key :&[u8], iv :&[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
    let mut retdata :Vec<u8> = encdata.to_vec();
    Aes256CfbDec::new(key.into(),iv.into()).decrypt(&mut retdata);
    Ok(retdata)
}