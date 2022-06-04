# easy_input_rust
###An easier way for user input process rust
The standard and built in way for getting input from user is so long, tough and maybe complicated
The standard way:
```
  std::io::stdin().read_line(&mut user_input).expect("failed to readline");
```
and the input has newline at the end of it so we have to trim it to ( if the OS is windows instead of \n it has \r\n at the end)
so I wrote this simple code that may help the beginners who want to start rust for the first time or even for the more advanced rust programmers.
Maybe it becomes useful a time when you are tired of writing a long code for getting a single input from user

simple example of using it
```
extern crate easy_user_input;
use easy_user_input::input;
fn main() {
    let user_input = input("enter a number").parse::<i32>();
    match user_input{
    Ok(num) => println!("{0} is a number", num),
    Err(not_num) => println!("{0} is not a number", not_num)
    };
}
