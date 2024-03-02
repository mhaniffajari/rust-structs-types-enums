fn ownership (){
    let numbers = vec![1,2,3,4,5];
    let slice = &numbers[0..4];
    println!("{:?}",slice);
}
fn modifable (){
    let mut numbers = vec![1,2,3];
    let slice  = &mut numbers[..];
    slice[0] = 20;
    println!("{:?}",slice);
}
fn main() {
    ownership();
    modifable();
}
