pub const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
pub fn variables() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
pub fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
