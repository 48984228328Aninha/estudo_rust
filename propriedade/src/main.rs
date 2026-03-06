fn main() {
    let s = String::from("Hello");

    let n: i32 = 5;


takes_ownership(s);
makes_copy(n);
}

fn takes_ownership(some_string: String) {
    println!("Some String: {some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("Some integer: {some_integer}");
}
