use std::cmp::Ordering;

pub fn turkish_char_order(c: char) -> usize {
    match c {
        'A' => 0,
        'a' => 1,
        'B' => 2,
        'b' => 3,
        'C' => 4,
        'c' => 5,
        'Ç' => 6,
        'ç' => 7,
        'D' => 8,
        'd' => 9,
        'E' => 10,
        'e' => 11,
        'F' => 12,
        'f' => 13,
        'G' => 14,
        'g' => 15,
        'Ğ' => 16,
        'ğ' => 17,
        'H' => 18,
        'h' => 19,
        'I' => 20,
        'ı' => 21,
        'İ' => 22,
        'i' => 23,
        'J' => 24,
        'j' => 25,
        'K' => 26,
        'k' => 27,
        'L' => 28,
        'l' => 29,
        'M' => 30,
        'm' => 31,
        'N' => 32,
        'n' => 33,
        'O' => 34,
        'o' => 35,
        'Ö' => 36,
        'ö' => 37,
        'P' => 38,
        'p' => 39,
        'Q' => 40,
        'q' => 41,
        'R' => 42,
        'r' => 43,
        'S' => 44,
        's' => 45,
        'Ş' => 46,
        'ş' => 47,
        'T' => 48,
        't' => 49,
        'U' => 50,
        'u' => 51,
        'Ü' => 52,
        'ü' => 53,
        'V' => 54,
        'v' => 55,
        'W' => 56,
        'w' => 57,
        'X' => 58,
        'x' => 59,
        'Y' => 60,
        'y' => 61,
        'Z' => 62,
        'z' => 63,
        _ => usize::MAX,
    }
}

pub fn turkish_collate(lhs: &str, rhs: &str) -> Ordering {
    for (lc, rc) in lhs.chars().zip(rhs.chars()) {
        match turkish_char_order(lc).cmp(&turkish_char_order(rc)) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }

    lhs.len().cmp(&rhs.len())
}

pub fn trim_string(s: &str, len: usize, wide: bool) -> String {
    if wide {
        s.to_string()
    } else {
        let end = s.char_indices().nth(len).unwrap_or((s.len(), '0')).0;
        s[..end].to_string()
    }
}
