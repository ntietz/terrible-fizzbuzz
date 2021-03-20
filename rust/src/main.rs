use unroll::unroll_for_loops;

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

fn print(x: usize) {
    let funcs = [
        print_num,
        print_fizz,
        print_buzz,
        print_fizzbuzz,
    ];

    funcs[(((x % 3) == 0) as usize) ^ ((((x % 5) == 0) as usize) << 1)](x);
}

#[unroll_for_loops]
fn main() {
    for x in 1..101 {
        print(x);
    }
}
