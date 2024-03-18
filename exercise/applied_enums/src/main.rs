enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}
fn format_size(size: u64) -> String {
    let filesize = match size{
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size/1000),
        1_000_000..=999_999_999 => FileSize::Megabytes(size/1_000_000),
        _ => FileSize::Gigabytes(size/1_000_000_000),
    };
    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{} kilobytes", kb),
        FileSize::Megabytes(mb) => format!("{} megabytes", mb),
        FileSize::Gigabytes(gb) => format!("{} gigabytes", gb),
    }
}

fn main() {
    let result = format_size(1000005);
    println!("{}", result);
}
