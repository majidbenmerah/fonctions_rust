// Pallindrome

fn is_pallindrome(mot:&str) -> bool {

    let inverse = mot.chars().rev().collect::<String>();
    if mot == inverse {
        true
    } else {
        false
    }
}
