use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let bytes = input.as_bytes();

    for i in 0..bytes.len() - 14 {
        let mut has_repeats = false;
        for j in i..i + 14 {
            for k in j + 1..i + 14 {
                if bytes[j] == bytes[k] {
                    has_repeats = true;
                }
            }
        }

        if !has_repeats {
            println!("{}", i + 14);
            break;
        }
    }
}
