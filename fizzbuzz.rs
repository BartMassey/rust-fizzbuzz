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

fn fizzbuzz_write<W: Write>(buf: &mut W) {
    for i in 1..100_000_00 {
        match (i % 3 == 0, i % 5 == 0) {
            (false, false) => {
                let mut s = i.to_string();
                s.push('\n');
                let _ = buf.write(s.as_bytes()).unwrap();
            }
            (true, false) => { let _ = buf.write(b"fizz\n").unwrap(); }
            (false, true) => { let _ = buf.write(b"buzz\n").unwrap(); }
            (true, true) => { let _ = buf.write(b"fizzbuzz\n").unwrap(); }
        }
    }
}


fn almost_slow() {
    let stdout = std::io::stdout();
    let mut buf = stdout.lock();
    fizzbuzz_write(&mut buf);
}

fn fast() {
    let stdout = std::io::stdout();
    let lock = stdout.lock();
    let mut buf = std::io::BufWriter::with_capacity(32 * 1024, lock);
    fizzbuzz_write(&mut buf);
}


fn main() {
    match std::env::args().nth(1).unwrap().as_str() {
        "--fast" => fast(),
        "--almost-slow" => almost_slow(),
        "--slow" => slow(),
        e => eprintln!("{}: fizzbuzz: usage: fizzbuzz --fast|--almost-slow|--slow", e),
    }
}
