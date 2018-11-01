use std::os::raw::c_char;

pub(crate) fn must_chars_to_string(chars: &[c_char]) -> String {
    let mut bytes: Vec<u8> = Vec::with_capacity(chars.len());

    for i in chars {
        if *i == 0 {
            break;
        }

        bytes.push(*i as u8);
    }

    String::from_utf8(bytes).expect("invalid utf8 string")
}

pub(crate) use std::slice::from_raw_parts;
