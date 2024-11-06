fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }
    let n = ((n % len) + len) % len; // Нормализуем сдвиг
    let split_point = (len - n) as usize;
    format!("{}{}", &s[split_point..], &s[..split_point])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts.iter().for_each(|(n, expected)| {
            assert_eq!(rotate(s.clone(), *n), expected.to_string());
        });
    }
}

fn main() {
    let s = "abcdefgh".to_string();
    let rotated = rotate(s.clone(), 2);
    println!("Original: {}", s);
    println!("Rotated by 2: {}", rotated);
}
