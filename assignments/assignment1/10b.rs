enum Color {
    Red,
    Yellow,
    Blue,
}

fn say_color(color: &Color) -> &'static str {
    match color {
        Color::Red => "red",
        Color::Yellow => "yellow",
    }
}

fn main() {
    println!("{}", say_color(&Color::Blue));
}
