use std::io;

fn main()
{

    println!("Podaj imie: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Błąd odczytu"); // .expect = wyjatek
    println!("Cześć, {}", input.trim());
}
