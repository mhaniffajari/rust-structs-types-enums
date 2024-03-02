#[derive(Debug)]

enum CoffeeRegions
{
    Java,
    Aceh,
    Ethiopia,
    Toraja
}

struct Coffee
{
    name : String,
    region : CoffeeRegions
}

fn suppoted_regions(c:CoffeeRegions)
{
    match c
    {
        CoffeeRegions::Java => println!("Java"),
        _ => println!("{:?} is Not Supported Region!",c)
    }
}

fn main()
{
    let coffe1 = Coffee{
        name : String::from("Preanger"),
        region : CoffeeRegions::Java
    };
    let coffe2 = Coffee{
        name : String::from("Gayo"),
        region : CoffeeRegions::Aceh
    };
    println!("Coffee 1 : {} from {:?}",coffe1.name,coffe1.region);
    println!("Coffee 2 : {} from {:?}",coffe2.name,coffe2.region);
    suppoted_regions(coffe1.region);
    suppoted_regions(coffe2.region);
    }