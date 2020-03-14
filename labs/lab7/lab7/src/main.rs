mod chebyshev_distance;
mod euclidean_distance;
mod manhattan_distance;
mod part2;
use std::io::{self, Write};

fn main() {
    println!("Part 2 output:");
    part2::main();
    println!("\n\nStarting CLI for computing distance...");
    user_input();
}

fn user_input() {
    loop {
        print!("Input the X coordinate of the first point: ");
        io::stdout().flush().unwrap();
        let p1_x = read_number();
        if p1_x.is_none() {
            continue;
        }
        let p1_x = p1_x.unwrap();

        print!("Input the Y coordinate of the first point: ");
        io::stdout().flush().unwrap();
        let p1_y = read_number();
        if p1_y.is_none() {
            continue;
        }
        let p1_y = p1_y.unwrap();

        print!("Input the X coordinate of the second point: ");
        io::stdout().flush().unwrap();
        let p2_x = read_number();
        if p2_x.is_none() {
            continue;
        }
        let p2_x = p2_x.unwrap();

        print!("Input the Y coordinate of the second point: ");
        io::stdout().flush().unwrap();
        let p2_y = read_number();
        if p2_y.is_none() {
            continue;
        }
        let p2_y = p2_y.unwrap();

        println!(
            "What kind of distance do you want to calculate?
               (1) Euclidean
               (2) Manhattan
               (3) Chebyshev"
        );
        let choice = read_number();
        if choice.is_none() {
            continue;
        }

        match choice {
            Some(1) => {
                let p1 = euclidean_distance::Point { x: p1_x, y: p1_y };
                let p2 = euclidean_distance::Point { x: p2_x, y: p2_y };
                println!(
                    "Euclidean Distance: {}",
                    euclidean_distance::compute_euclidean_distance(&p1, &p2)
                );
            }
            Some(2) => {
                let p1 = manhattan_distance::Point { x: p1_x, y: p1_y };
                let p2 = manhattan_distance::Point { x: p2_x, y: p2_y };
                println!(
                    "Manhattan Distance: {}",
                    manhattan_distance::compute_manhattan_distanceC(&p1, &p2)
                );
            }
            Some(3) => {
                let p1 = chebyshev_distance::Point { x: p1_x, y: p1_y };
                let p2 = chebyshev_distance::Point { x: p2_x, y: p2_y };
                println!(
                    "Chebyshev Distance: {}",
                    chebyshev_distance::compute_chebyshev_distanceC(&p1, &p2)
                );
            }
            _ => println!("Error: invalid option selected, try again"),
        }
    }
}

fn read_number() -> Option<i8> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => match buffer.trim_end().parse() {
            Ok(n) => Some(n),
            Err(e) => {
                println!("Error: {}:", e);
                None
            }
        },
        Err(e) => {
            println!("Error: {}:", e);
            None
        }
    }
}
