/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let chars:Vec<char> = code.chars().filter(|x| (*x).is_digit(32)).collect();
    let digits:Vec<u32> = chars.iter().map(|x| x.to_digit(32).unwrap()).collect();
    let mut count:u32 = 0;
    let mut current_index: u32 = 0;

    for x in digits.iter().rev(){
        current_index = current_index + 1;
        if current_index % 2 == 0{
            if (x*2) > 9{count = count + (x*2 - 9);}
            else { count = count + (x*2) }
        }else { count = count + x;}
    }
    if count % 10 == 0{ return true;}
    else { return false; }
}