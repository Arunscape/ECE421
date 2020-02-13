mod combination;
mod tax;

fn main() {
    println!("{}", tax::taxed_amount(10000000.0));
}
