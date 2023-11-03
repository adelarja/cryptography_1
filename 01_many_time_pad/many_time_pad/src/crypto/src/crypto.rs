use std::cmp;
use hex;

pub fn xor_bytes(arr1: &Vec<u8>, arr2: &Vec<u8>) -> Vec<u8> {
    arr1.iter().zip(arr2).map(|(x, y)| x ^ y).collect()
}

pub fn and_bytes(arr1: &Vec<u8>, arr2: &Vec<u8>) -> Vec<u8> {
    arr1.iter().zip(arr2).map(|(x, y)| x & y).collect()
}

pub fn mix_key_values(key1: &Vec<u8>, key2: &Vec<u8>) -> Vec<u8> {
    let mut key: Vec<u8> = vec![];

    for (k1, k2) in key1.iter().zip(key2) {
        if k1 != &0 {
            key.push(*k1);
        } else if k2 != &0 {
            key.push(*k2);
        } else {
            key.push(0)
        }
    }

    key
}

pub fn hex_to_bytes_vector(data: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(data)
}

pub fn align_messages(str1: &str, str2: &str) -> (String, String) {
    let length = match str1.len().cmp(&str2.len()) {
        cmp::Ordering::Less => str1.len(),
        _ => str2.len()
    };
    
    (String::from(&str1[..length]), String::from(&str2[..length]))
}

pub fn space_mask(ciphered1: &Vec<u8>, ciphered2: &Vec<u8>) -> Vec<u8> {
    let mixed_messages = xor_bytes(ciphered1, ciphered2);
    let mut mask: Vec<u8> = vec![];

    if ciphered1 == ciphered2 {
        for _ in 0..ciphered1.len() {
            mask.push(1);
        }
        return mask;
    }

    for value in mixed_messages {
        match char::from_digit(value as u32, 36) {
            Some(_) => mask.push(1),
            _ => {
                match value {
                    18..=57 => mask.push(1),
                    _ => mask.push(0)

                }
            }
        }
    }

    mask
}

pub fn validate_space(ciphered: &Vec<u8>, validation_group: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut mask = space_mask(ciphered, &validation_group[0]);

    for ciphered2 in validation_group {
        let new_mask = space_mask(ciphered, ciphered2);
        mask = and_bytes(&mask, &new_mask);
    }
    
    mask
}

pub fn get_key_values(ciphered: &Vec<u8>, mask: &Vec<u8>) -> Vec<u8> {
    let mut key: Vec<u8> = vec![];

    for (m, c) in mask.iter().zip(ciphered) {
        if m == &1 {
            key.push(c ^ 32);
        } else {
            key.push(0);
        }
    }

    key
}

pub fn expand_word_to_length(word: &str, length: usize) -> Vec<u8> {
    let original_length = word.len();
    if original_length >= length {
        return word.as_bytes().to_vec();
    }

    let mut expanded = String::with_capacity(length);
    while expanded.len() < length {
        expanded.push_str(word);
    }

    if expanded.len() > length {
        expanded.truncate(length);
    }

    expanded.as_bytes().to_vec()  
}
