enum DiskType {
    CD,
    DVD,
}
#[derive(Debug)]
enum DiskSize {
    BYTE(u32),
    KB(u32),
    MB(u32),
    GB(u32),
}
fn main() 
{
    let disk_type = DiskType::CD;
    match disk_type {
        DiskType::CD => println!("The disk type is CD"),
        DiskType::DVD => println!("The disk type is DVD"),
    }
    let disk_size = DiskSize::BYTE(700);
    println!("{:?}",disk_size);
}