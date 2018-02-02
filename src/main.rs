use std::{i8, i16, isize, usize, f32};

fn main() {

    println!("\n");

    println!("Hello, world!");

    println!("\n");

    math();

    println!("\n");

    variables();

    println!("\n");


    println!("12 + 8 = {}", get_sum(12,8));

    println!("\n");

    let sum = get_sum;
    println!("92 + 8 = {}", sum(92,8));

    println!("\n");

}

fn math() {
    println!("5 + 4 = {}", 5 + 4);
    println!("5 - 4 = {}", 5 - 4);
    println!("5 / 4 = {}", 5 / 4);
    println!("5 * 4 = {}", 5 * 4);
    println!("5 % 4 = {}", 5 % 4);

    let neg_4 = -4i32;

    println!("abs(-4) = {}", neg_4.abs());
}


fn variables() {
    //let x = 5;
    //println!("The value of x is: {}", x);
    //let mut x = 6;
    //println!("The value of x is: {}", x);

    println!("max i8 {}", i8::MAX);
    println!("max i8 {}", i8::MIN);
    println!("max i16 {}", i16::MAX);
    println!("max i16 {}", i16::MIN);
    println!("max isize {}", isize::MAX);
    println!("max isize {}", isize::MIN);
    println!("max usize {}", usize::MAX);
    println!("max usize {}", usize::MIN);
    println!("max f32 {}", f32::MAX);
    println!("max f32 {}", f32::MIN);
    let age = 1;
    let is_it_true: bool = true;
    let _x = 'x';
    println!("i am {} year old", age);

    let (f_name, l_name) = ("jernej", "uranic");

    println!("it is {0} that {1} is {0}", is_it_true, _x);

    println!("{:.2}", 1.234);
    println!("B: {:b} H: {:x} O: {:o}", 10, 10, 10);

    println!("{ten:>ws$}", ten=10, ws=5);

    println!("{ten:>0ws$}", ten=10, ws=5);

}

fn get_sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
