fn main() {
    variables();
    shadowing();
}

fn variables() {
    let mut x = 5; // immutable variable
    println!("The value of x is: {x}");
    x = 6; // error: cannot assign twice to immutable variable `x`
    println!("The value of x is: {x}");

    const MAX_RETRIES: u32 = 3; // constant
    // cannot use mut with const
    // const mut MAX_RETRIES: u32 = 3; // error: expected one of `!`, `(`, `+`, `::`, `;`, `<`, or `=`, found `mut`
    println!("The value of MAX_RETRIES is: {MAX_RETRIES}");
}


fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2; // shadowing but not mutate the outer x
        println!("The value of x in inner scope is: {x}"); // 12
    }
    println!("The value of x is: {x}"); // 6
}