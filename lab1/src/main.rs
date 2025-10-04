use std::{fs::File, io};
use rand::Rng;
use std::io::prelude::*;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn powers(number: u64) -> [u64; 10] {
    let mut array = [number; 10];
    for i in 1 .. 10 {
        array[i] = array[i - 1] * number;
    }

    array
}

fn test_collatz_on_array(array: [u64; 10]) -> [bool; 10] {
    let mut result = [false; 10];
    
    for i in 0 .. 10 {
        let mut testing_number = array[i];
        for _j in 0 .. 100 {
            if testing_number % 2 == 0 {
                testing_number /= 2;
            }
            else {
                testing_number = testing_number * 3 + 1;
            }

            if testing_number == 1
            {
                result[i] = true;
                break;
            }
        }
    }

    result
}

fn collatz_shenanigans(x: u64) {
    let x_powers = powers(x);
    let collatz_test = test_collatz_on_array(x_powers);

    let mut file = File::create("xyz.txt").expect("Couldn't open file");
    for item in &collatz_test {
        if *item {
            file.write_all(b"true ").expect("Couldn't write to file");
        }
        else {
            file.write_all(b"false ").expect("Couldn't write to file");
        }
    }
}

fn read_tuple() -> (u16, String) {
    let input_number = read_input();

    let number = input_number.trim().parse::<u16>().unwrap_or(42);

    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    (number, input_string)
}

fn digit_to_string(digit: u16) -> String {
    match digit {
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        _ => String::from("something went wrong"),
    }
}

fn weird_tuple_functionality() {
    'outer: loop {
        println!("Podaj liczbę powtórzeń");

        let input = read_input();
        let iterations = input.trim().parse::<u64>().unwrap_or_default();

        for _i in 0 .. iterations {
            println!("Podaj cyfrę i jej angielską nazwę, dobra kombinacja skończy program");
            let (number, translation) = read_tuple();
            if translation.trim() == digit_to_string(number) {
                break 'outer;
            }
        }
    }
}

fn main() {
    let returned = loop {
        println!("Podaj liczbę");

        let guess = read_input();

        let mut x = match guess.trim().parse::<u64>() {
            Ok(number) => number,
            Err(_) => break true,
        };

        if x == 0 {
            break false;
        }
            
        let random_number: u64 = rand::rng().random_range(0 ..= 5);
        x += random_number; 
        println!("New x value: {x}");

        collatz_shenanigans(x);
    };

    match returned {
        true => println!("Ended because of error"),
        false => println!("Ended because of user's choice")
    }

    weird_tuple_functionality();
}
