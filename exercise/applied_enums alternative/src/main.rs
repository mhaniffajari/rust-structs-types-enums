enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

impl FileSize {
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB", *kb as f64 / 1000.0),
            FileSize::Megabytes(mb) => format!("{:.2} MB", *mb as f64 / 1_000_000.0),
            FileSize::Gigabytes(gb) => format!("{:.2} GB", *gb as f64 / 1_000_000_000.0),
        }
    }
}

fn main() {
    let size = 200;
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1_000..=999_999 => FileSize::Kilobytes(size / 1_000),
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        _ => FileSize::Gigabytes(size / 1_000_000_000),
    };
    println!("{}", filesize.format_size());
}
