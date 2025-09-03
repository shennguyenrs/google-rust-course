// When will this function panic?
// When received n is not type u32, for example, signed integer, larger than u32 limit, floating
// point number
fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    let n = 5;
    println!("fib({n}) = {}", fib(n));
}
