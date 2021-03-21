use std::process::exit;

fn print_num(x: usize) {
    println!("{}", x);
}

fn print_fizz(_: usize) {
    println!("fizz");
}

fn print_buzz(_: usize) {
    println!("buzz");
}

fn print_fizzbuzz(_: usize) {
    println!("fizzbuzz");
}

fn handle(x: usize) {
    let printers = [print_num, print_fizz, print_buzz, print_fizzbuzz];
    printers[(((x % 3) == 0) as usize) ^ ((((x % 5) == 0) as usize) << 1)](x);
}

fn main() {
    // look, I know variables are supposed to have good names, and I happen to
    // think that winston is a good name.
    let winston = [|| {}, || { exit(0) }];

    let mut x = 1;

    loop {
        handle(x);
        winston[(x == 100) as usize]();
        x += 1;
    }
}
