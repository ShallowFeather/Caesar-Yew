pub fn encrypt(s: String, num: u8) -> String {
    let mut res: Vec<char> = vec![' '; s.len()];
    for (i, ch) in s.chars().enumerate() {
        if ch >= 'A' && ch <= 'Z' {
            res[i] = (((((ch as u8) - ('A' as u8)) + num) % 26) + ('A' as u8)) as char;
        }
        else if ch <= 'z' && ch >= 'a' {
            res[i] = (((((ch as u8) - ('a' as u8)) + num) % 26) + ('a' as u8)) as char
        }
    }
    res.into_iter().collect()
}

pub fn decrypt(s: String, num: u8) -> String {
    let mut res: Vec<char> = vec![' '; s.len()];
    for (i, ch) in s.chars().enumerate() {
        if ch >= 'A' && ch <= 'Z' {
            res[i] = (((((ch as u8) - ('A' as u8)) - num) % 26) + ('A' as u8)) as char;
        }
        else if ch <= 'z' && ch >= 'a' {
            res[i] = (((((ch as u8) - ('a' as u8)) - num) % 26) + ('a' as u8)) as char
        }
    }
    res.into_iter().collect()
}