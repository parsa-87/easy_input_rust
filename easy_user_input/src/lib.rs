
use std::io

pub fn input(str: String) -> String {
    //remove \r\n from string
    fn trim_newline(s: String) -> String{
        let mut return_str = String::from(s);
        if return_str.ends_with('\n') {
            return_str.pop();
            if return_str.ends_with('\r') {
                return_str.pop();
            }
        }
        return_str
    }
    let mut user_input = String::new();
    //entery
    println!(str);
    io::stdin().read_line(&mut user_input).expect("failed to readline");
    trim_newline(user_input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
