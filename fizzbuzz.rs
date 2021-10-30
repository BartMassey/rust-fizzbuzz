use std::io::Write;

fn slow() {
    for i in 1..100_000_00 {
        match (i % 3 == 0, i % 5 == 0) {
            (false, false) => println!("{}", i),
            (true, false) => println!("fizz"),
            (false, true) => println!("buzz"),
            (true, true) => println!("fizzbuzz"),
        }
    } 
}

fn medium() {
    let stdout = std::io::stdout();
    let mut buf = stdout.lock();
    for i in 1..100_000_00 {
        match (i % 3 == 0, i % 5 == 0) {
            (false, false) => {
                let _ = buf.write(i.to_string().as_bytes()).unwrap();
                let _ = buf.write(b"\n").unwrap();
            }
            (true, false) => { let _ = buf.write(b"fizz\n").unwrap(); }
            (false, true) => { let _ = buf.write(b"buzz\n").unwrap(); }
            (true, true) => { let _ = buf.write(b"fizzbuzz\n").unwrap(); }
        }
    } 
}

fn fast() {
    let stdout = std::io::stdout();
    let lock = stdout.lock();
    let mut buf = std::io::BufWriter::with_capacity(32 * 1024, lock);
    for i in 1..100_000_00 {
        match (i % 3 == 0, i % 5 == 0) {
            (false, false) => {
                let _ = buf.write(i.to_string().as_bytes()).unwrap();
                let _ = buf.write(b"\n").unwrap();
            }
            (true, false) => { let _ = buf.write(b"fizz\n").unwrap(); }
            (false, true) => { let _ = buf.write(b"buzz\n").unwrap(); }
            (true, true) => { let _ = buf.write(b"fizzbuzz\n").unwrap(); }
        }
    } 
}

fn main() {
    match std::env::args().nth(1).unwrap().as_str() {
        "--fast" => fast(),
        "--medium" => medium(),
        "--slow" => slow(),
        e => eprintln!("{}: fizzbuzz: usage: fizzbuzz --fast|--slow", e),
    }
}
