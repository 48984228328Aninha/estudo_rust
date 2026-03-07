fn main() {
    let s = String::from("Hello");

    let s1 = String::from("Ola");
    let (s2, len) = calculate_length(s1);

    let n: i32 = 5;

    let s1 = gives_ownership();
    let s2 = String::from("olá");
    let s3 = takes_and_gives_back(s2);

    println!("s1: {s1}, s3: {s3}");


takes_ownership(s);
makes_copy(n);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
} 

fn takes_ownership(some_string: String) {
    println!("Some String: {some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("Some integer: {some_integer}");
}

fn gives_ownership() -> String {
    let alguma_string = String::from("Olá");
    alguma_string
}

fn takes_and_gives_back(string: String) -> String {
    string
}
