struct Color
{

    RED: u8,
    GREEN: u8,
    BLUE: u8
}

fn main()
{

    let Blue = Color {RED: 0, GREEN: 0, BLUE: 255};
    // Blue.RED = 5; with MUT

    PRINT_COLOR(&Blue);
}

fn PRINT_COLOR(c: &Color)
{

    println!("R: {}, G: {}, B: {}", c.RED, c.GREEN, c.BLUE);
}
