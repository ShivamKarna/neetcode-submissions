impl Solution{
pub fn is_palindrome(s: String) -> bool {

    let chars = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<char>>();
    
    let mut left = 0;
    let mut right = chars.len().saturating_sub(1);

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    return true;
}
}