fn main() {
    use clap::{load_yaml, App};
    let yml = load_yaml!("cli.yml");
    let m = App::from(yml).get_matches();

    println!("{:?}", m);
}
