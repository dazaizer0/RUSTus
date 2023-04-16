enum Direction
{

    Up,
    Down,
    Right,
    Left
}

fn main()
{

    let DIRECTION:Direction = Direction::Right;

    match DIRECTION
    {

        Direction::Up =>println!("Up"),
        Direction::Down =>println!("Down"),
        Direction::Right =>println!("Right"),
        Direction::Left =>println!("Left"),
    }
}
