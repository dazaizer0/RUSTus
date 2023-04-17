struct Color(u8, u8, u8);
fn main()
{

    let mut RED = Color(255, 0, 0,);
    RED.2 = 55;

    println!("COLOR: {}, {}, {}", RED.0, RED.1, RED.2);
}