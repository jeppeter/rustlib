//! AES256 CBC、CTR mode encrypt decrypt demo
extern crate crypto;
use std::str;
use crypto::{buffer,aes,blockmodes,aessafe};
use crypto::buffer::{ReadBuffer,WriteBuffer,BufferResult};
use crypto::symmetriccipher::{BlockEncryptor,BlockDecryptor};
//use rand::{Rng};
//use rand::rngs::{OsRng};

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

use std::error::Error;

extargs_error_class!{AesLibError}


// Encrypt a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
pub fn aes256_cbc_encrypt(data: &[u8],key: &[u8], iv: &[u8])->Result<Vec<u8>,Box<dyn Error>>{
    let mut encryptor=aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result=Vec::<u8>::new();
    let mut read_buffer=buffer::RefReadBuffer::new(data);
    let mut buffer=[0;4096];
    let mut write_buffer=buffer::RefWriteBuffer::new(&mut buffer);

    loop{
        let ro=encryptor.encrypt(&mut read_buffer,&mut write_buffer,true);
        if ro.is_err() {
            let e = ro.err().unwrap();
            extargs_new_error!{AesLibError,"encrypt error [{:?}]",e}
        }
        let result = ro.unwrap();

        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow=>break,
            BufferResult::BufferOverflow=>{},
        }
    }

    Ok(final_result)
}

// Decrypts a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
pub fn aes256_cbc_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let ro = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);
        if ro.is_err() {
            let e = ro.err().unwrap();
            extargs_new_error!{AesLibError,"decrypt error [{:?}]",e}
        }
        let result = ro.unwrap();
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
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

    let aes_enc = aessafe::AesSafe128Encryptor::new(key);
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

    let aes_dec = aessafe::AesSafe128Decryptor::new(key);
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

    let aes_enc = aessafe::AesSafe192Encryptor::new(key);
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

    let aes_dec = aessafe::AesSafe192Decryptor::new(key);
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

    let aes_enc = aessafe::AesSafe256Encryptor::new(key);
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

    let aes_dec = aessafe::AesSafe256Decryptor::new(key);
    let mut retdata :Vec<u8> = Vec::new();
    for _ in 0..(encdata.len()-8) {
        retdata.push(0);
    }
    aes_dec.decrypt_block(&encdata[..], &mut retdata[..]);
    Ok(retdata)
}