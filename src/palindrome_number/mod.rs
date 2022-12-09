pub fn is_palindrome(x: i32) -> bool {
    let s: String = x.to_string();
    let rev: String = s.chars().rev().collect::<String>();
    s.eq_ignore_ascii_case(&rev)
}