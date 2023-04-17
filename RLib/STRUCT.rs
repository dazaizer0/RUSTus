struct Pokemon
{

    Charizard: u8,
    Pikachu: u8,
    Squirtle: u8
}

fn main()
{

    let mut POKEMON = Pokemon {Charizard: 1, Pikachu: 5, Squirtle: 3};
    POKEMON.Pikachu = 2;

    println!("{}, {}, {}", POKEMON.Charizard, POKEMON.Pikachu, POKEMON.Squirtle);
}
