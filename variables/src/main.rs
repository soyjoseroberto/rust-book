fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Example of shadowing where you avoid using spaces_str and spaces_num as var names
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is {}", spaces);

    // A statement does something but does not return a value
    // An expression returns a value
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    // Countdown before rocket launch (reverse func)
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // Chapter 4: Ownership
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}
