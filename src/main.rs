use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut b = Vec::new();

    f.read_to_end(&mut b)?;

    let mut i = 0;

    while i + 3 < b.len() - 1 {
        if b[i] == b[i + 1] || b[i] == b[i + 2] || b[i] == b[i + 3] {
            i += 1;
        } else if b[i + 1] == b[i + 2] || b[i + 1] == b[i + 3] {
            i += 2;
        } else if b[i + 2] == b[i + 3] {
            i += 3;
        } else {
            println!("{}", i + 4);
            return Ok(());
        }
    }
    Ok(())
}
