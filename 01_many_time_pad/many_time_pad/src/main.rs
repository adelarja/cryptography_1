use hex;
use std::cmp::Ordering;
use std::str;

const C1: &str = "315c4eeaa8b5f8aaf9174145bf43e1784b8fa00dc71d885a804e5ee9fa40b16349c146fb778cdf2d3aff021dfff5b403b510d0d0455468aeb98622b137dae857553ccd8883a7bc37520e06e515d22c954eba5025b8cc57ee59418ce7dc6bc41556bdb36bbca3e8774301fbcaa3b83b220809560987815f65286764703de0f3d524400a19b159610b11ef3e";
const C2: &str = "234c02ecbbfbafa3ed18510abd11fa724fcda2018a1a8342cf064bbde548b12b07df44ba7191d9606ef4081ffde5ad46a5069d9f7f543bedb9c861bf29c7e205132eda9382b0bc2c5c4b45f919cf3a9f1cb74151f6d551f4480c82b2cb24cc5b028aa76eb7b4ab24171ab3cdadb8356f";

fn main() {
    // println!("{:?}", str::from_utf8(&xored).unwrap());

    let (m1, m2) = align_messages(C1, C2);
    let xored = xor_hex_messages(m1, m2);
    println!(
        "{:?}",
        String::from_utf8_lossy(&xored)
            .split("")
            .collect::<Vec<_>>()
    );

    let chars: Vec<_> = find_characters(C1, vec![C2]);
    println!("{:?}", chars);

    // " " ^ "a" -> "A"
    // " " ^ "A" -> "a"
}

/// Perform the xor between message_1 and message_2 and return the result as a vector of bytes.
fn xor_hex_messages(message_1: &str, message_2: &str) -> Vec<u8> {
    let (message_1, message_2) = align_messages(message_1, message_2);

    let m1_bytes = hex::decode(message_1).expect("Invalid hex.");
    let m2_bytes = hex::decode(message_2).expect("Invalid hex.");

    let xored: Vec<_> = m1_bytes.iter().zip(m2_bytes).map(|(x, y)| x ^ y).collect();
    xored
}

/// Perform the slice of the messages, using the len of the shorter string as upper limit.
fn align_messages<'a>(message_1: &'a str, message_2: &'a str) -> (&'a str, &'a str) {
    let limit = match message_1.len().cmp(&message_2.len()) {
        Ordering::Greater => message_2.len(),
        _ => message_1.len(),
    };

    (&message_1[..limit], &message_2[..limit])
}

fn find_characters(message: &str, messages: Vec<&str>) -> Vec<Vec<u8>> {
    let xored: Vec<_> = messages
        .iter()
        .map(|x| xor_hex_messages(message, x))
        .collect();
    let filtered: Vec<_> = xored
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| if char::from(*y).is_digit(36) { *y } else { 0 })
                .collect::<Vec<_>>()
        })
        .collect();
    filtered
}
