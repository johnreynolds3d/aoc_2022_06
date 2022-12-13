use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::Read;

// from: https://stackoverflow.com/a/46767732/2017082
fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn main() -> io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut b = Vec::new();

    f.read_to_end(&mut b)?;

    let mut i = 0;
    let p = 14; // use '4' for Part 1

    while i + p - 1 < b.len() - 1 {
        let slice = &b[i..i + p];
        if has_unique_elements(slice) {
            println!("{}", i + p);
            return Ok(());
        }
        i += 1;
    }
    Ok(())
}
