// fn get_index()
// {
//     // let index = 3;
//     // let vec = vec![1,2,3,4];
//     let value = vec.get(index).unwrap();
//     println!("the value at index {} is {:?}",index,value);
// }

fn get_index(index: usize, vec: &Vec<i32>) {
    // Check if the index is within the bounds of the vector
    if let Some(value) = vec.get(index) {
        println!("The value at index {} is {:?}", index, value);
    } else {
        println!("Index out of bounds");
    }
}
fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7];
    get_index(2, &vec);
    // let second_value = vec[1];
    // println!("{}",second_value);
    // let last_value = vec.last().unwrap();
    // println!("{}",last_value);
    // match vec.first()
    // {
    //     Some(first_value) => println!("the first value : {}",first_value),
    //     None => println!("No value found"),
    // }
}