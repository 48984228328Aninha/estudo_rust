fn main() {
    let s1 = String::from("Ola");
    let len = calculate_length(&s1);

    let s2 = String::from("iai :)")
    let len2 = calculate_length(&s2);

    println!("The size of {s1} is: {len}");

    println!("the size of {s1} is: {len}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

