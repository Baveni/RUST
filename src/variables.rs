use std::{i8, i16, isize, usize, f32};
//use std::io::stdin;

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