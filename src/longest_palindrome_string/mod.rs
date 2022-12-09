pub fn longest_palindrome(s: String) -> String {
    //manacher's algorithm
    let mut t = String::new();
    t.push('$');
    t.push('#');
    for c in s.chars() {
        t.push(c);
        t.push('#');
    }
    t.push('^');
    let mut p = vec![0; t.len()];
    let mut c = 0;
    let mut r = 0;
    for i in 1..t.len() - 1 {
        let i_mirror = 2 * c - i;
        if r > i {
            p[i] = std::cmp::min(r - i, p[i_mirror]);
        }
        while t.chars().nth(i + 1 + p[i]).unwrap() == t.chars().nth(i - 1 - p[i]).unwrap() {
            p[i] += 1;
        }
        if i + p[i] > r {
            c = i;
            r = i + p[i];
        }
    }
    let mut max_len = 0;
    let mut center_index = 0;
    for i in 1..t.len() - 1 {
        if p[i] > max_len {
            max_len = p[i];
            center_index = i;
        }
    }
    let start = (center_index - 1 - max_len) / 2;
    let end = start + max_len;
    s[start..end].to_string()
}
