// WHILE
fn main()
{

    let mut n = 0;

    while n <= 100
    {

        if n % 2 == 0
        {

            println!("N: {}", n);
        }

        n += 1;
    }
}
                 
// FOR
fn main()
{

    for i in 1..10
    {
        
        println!("NR: {}", i);
    }

}

fn main()
{

    let items = vec!["PHONE", "SWITCH", "COMPUTER"];
    for item in items.iter()
    {

        println!("item form items: {}", item);
    }
}

fn main()
{

    let items = vec!["PHONE", "SWITCH", "COMPUTER"];

    for (index, item) in items.iter().enumerate()
    {

        println!("item form items: {}.{}", index, item);
    }
}

