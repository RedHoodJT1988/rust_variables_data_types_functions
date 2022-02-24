fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // Data types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{}", x);
    println!("{}", y);

    // Numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    println!("add {}", sum);
    println!("subtraction {}", difference);
    println!("multiplication {}", product);
    println!("division {}", quotient);
    println!("floor division {}", floored);
    println!("remainder {}", remainder);

    // boolean values
    let t = true;

    let f: bool = false;
    println!("True {}", t);
    println!("False {}", f);

    // Characters
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    println!("lower case letter {}", c);
    println!("upper case letter {}", z);
    println!("Emoji {}", heart_eyed_cat);

    // Compound types
    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // using the period to access a tuple element directly
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("Tuple element 1: {}", five_hundred);
    println!("Tuple element 2: {}", six_point_four);
    println!("Tuple element 3: {}", one);

    // Arrays
    let _a = [1, 2, 3, 4, 5];

    let _months = ["January", "February", "March", "April", "May", "June", "July", "August",
        "September", "October", "November", "December"];

    // declare an array with the type and amount of elements
    let _b: [i32; 5] = [1, 2, 3, 4, 5];

    // Accessing elements in the array using indexing
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("First array element: {}", first);
    println!("Second array element: {}", second);

    another_function();
    print_labeled_measurement(5, 'h');
    let x = five();
    println!("The value of x is: {}", x);
}

// functions
fn another_function() {
    println!("Another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// function returning a value example
fn five() -> i32 {
    5
}

// Another example of returns
fn plus_one(x: i32) -> i32 {
    x + 1
}

