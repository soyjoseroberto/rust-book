fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Example of shadowing where you avoid using spaces_str and spaces_num as var names
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is {}", spaces);
    
}
