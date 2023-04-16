// int
fn main()
{

    let numbers = vec![1, 2, 3, 4, 5];
    let sum = sum_numbers(&numbers);

    println!("Sum of {:?} is {}", numbers, sum);
}

fn sum_numbers(numbers: &[i32]) -> i32
{

    let mut sum = 0;

    for &number in numbers
    {

        sum += number;
    }
    sum
}

// void
fn main()
{

    cout_numbers_range(1, 5);
}

fn cout_numbers_range(nmin: u32,nmax: u32)
{

    for i in nmin..nmax
    {

        println!("{}", i);
    }
}
