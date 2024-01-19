mod structs;

extern crate rand;

use rand::Rng;
use std::io::stdin;
use structs::MineField;

fn main() {
    let mut mine_field = MineField {
        size: (3, 3),
        mines: Vec::new(),
    };

    &mine_field.generate_mines();

    &mine_field.print_fields();

    let mut x: u32 = 0;
    let mut y: u32 = 0;

    println!("You have 5 tries");
    let mut counter = 1;
    loop {
        if counter == 5 {
            println!("Congrats, you won!");
            break;
        }

        println!("Enter x: ");
        let mut x_input = String::new();
        stdin().read_line(&mut x_input);
        x_input = x_input.trim().to_string();

        match x_input.parse::<u32>() {
            Ok(i) => x = i,
            Err(..) => println!("Not a number: {}", x_input),
        }

        println!("Enter y: ");
        let mut y_input = String::new();
        stdin().read_line(&mut y_input);
        y_input = y_input.trim().to_string();

        match y_input.parse::<u32>() {
            Ok(i) => y = i,
            Err(..) => println!("Not a number: {}", y_input),
        }

        if mine_field.find_by_coordinates(x, y) {
            println!("You hit a mine!");

            break;
        }
        counter += 1;
    }

    println!("The mines are marked with o:");
    mine_field.print_field_solved();
}
