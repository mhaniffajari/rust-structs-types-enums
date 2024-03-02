fn main(){
    let mut v = vec![1,3,5,7];
    // append the back value
    v.push(9);
    println!("{:?}",v);
    // add vector
    let add_vector = vec![11,13,15];
    v.extend(add_vector);
    println!("{:?}",v);
    // append vector
    let mut new_vector = vec![17,19];
    v.append(&mut new_vector);
    println!("{:?}",v);
    // insert value with specific index
    v.insert(0,0);
    println!("{:?}",v);
}