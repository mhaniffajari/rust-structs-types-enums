fn divide (x: i32, y: i32) -> Option<i32>
{
    if y == 0
    {
        None
    }
    else 
    {
        Some(x/y)
    }
}

fn main()
{
    let a = 32;
    let b  = 0;

    let result = divide(a,b);
    match result
    {
        Some(x) => println!("Result : {}",x),
        None => println!("Error : Division by Zero")
    }
}
