use std::collections::HashMap;

const KB_SHARING: &str = r#"
12345|67890-
QWERT|YUIOP
ASDFG|HJKL
ZXCVB|NM,.
"#;

#[derive(PartialEq)]
pub enum Player {
    Left,
    Right,
}

pub fn key_char_to_num(c: char) -> Option<u8> {
    match c {
        ('0'..='9') => Some((c as u8) - ('0' as u8) + 33),
        ('A'..='Z') => Some((c as u8) - ('A' as u8) + 7),
        '-' => Some(49),
        ',' => Some(51),
        '.' => Some(50),
        _ => None,
    }
}

pub fn key_num_to_char(num: u8) -> Option<char> {
    match num {
        33..=42 => Some((num - 33 + ('0' as u8)) as char),
        7..=32 => Some((num - 7 + ('a' as u8)) as char),
        49 => Some('-'),
        51 => Some(','),
        50 => Some('.'),
        _ => None,
    }
}

pub fn get_sharing() -> HashMap<u8, Player> {
    let mut res = HashMap::new();

    for line in KB_SHARING.split("\n") {
        if line.is_empty() {
            continue;
        }

        let sep_pos = line.find('|').unwrap();
        let left = &line[..sep_pos];
        let right = &line[sep_pos + 1..];

        for c in left.chars() {
            res.insert(key_char_to_num(c).unwrap(), Player::Left);
        }

        for c in right.chars() {
            res.insert(key_char_to_num(c).unwrap(), Player::Right);
        }
    }

    res
}
