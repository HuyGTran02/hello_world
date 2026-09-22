fn main() {
    let test1 = "ABC123DEF".to_string();
    let test2 = "123ABC".to_string();
    // Assertions for the test cases
    assert_eq!(q1_parser(test1), true);
    assert_eq!(q1_parser(test2), false);
    }


    // Utility method that takes a character and returns true if digit
    // or upper case letter.
    fn is_uppercase_or_digit(c: char) -> bool {
        is_uppercase_letter(c) || (c >= '0' && c <= '9')    
        }

    // Utility method that takes a character and returns true if it is
    // an upper case letter.
    fn is_uppercase_letter(c: char) -> bool {
        c >= 'A' && c <= 'Z'
        }

    fn q1_parser(text: String) -> bool {
    // make String into char vector
        let characters_array: Vec<char> = text.chars().collect();
    // for each character in the character vector do...
    // where i starts at 1 and increments for each iteration
        if characters_array.len() < 2 {
                return false;
            }
        for(i, character) in characters_array.iter().enumerate() {
    // YOUR CODE GOES HERE
            if i < 2 && !is_uppercase_letter(*character) {
                return false;
            }
            if i >= 2 && !is_uppercase_or_digit(*character) {
                return false;
            }
        }
        true
    }

fn chapter1_examples() {
    println!("Hello, world!");
}

fn chapter2_examples() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
}

fn chapter3_examples() {
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');

    stuff.0 += 3;

    let first_item = stuff.0;
    println!("The first item is: {}", first_item);

    let (a, b, c) = stuff;
    println!("b is {}", b);
}

fn chapter4_examples() {
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");

    fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        celsius * 1.8 + 32.0
    }
}

fn chapter5_examples() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];

    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    max = numbers[0];
    min = numbers[0];
    mean = 0.0;

    for &num in numbers.iter() {
        if num > max {
            max = num;
        }

        if num < min {
            min = num;
        }

        mean += num as f64;
    }

    mean /= numbers.len() as f64;
}