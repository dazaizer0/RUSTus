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
