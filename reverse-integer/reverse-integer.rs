impl Solution {
    pub fn reverse(x: i32) -> i32 {
    let mut string_x = x.to_string();
    let mut negative = false;
    if x < 0 {
        negative = true;
        string_x.remove(0);
    }
    let mut reverse_x = string_x.chars().rev().collect::<String>();
    if negative {
        reverse_x.insert(0, '-');
    }
    let my_int = match reverse_x.parse::<i32>(){
        Ok(n) => n,
        Err(_) => {
            println!("Failed to parse integer");
            0
        },
    };
    my_int
}

}