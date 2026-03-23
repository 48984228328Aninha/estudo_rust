fn main() {
    let s1 = String::from("Ola");
    let len = calculate_length(&s1);

    let s2 = String::from("iai :)")
    let len2 = calculate_length(&s2);

    let mut s = String::from("hello");

    modifica(&mut s);

    println!("The size of {s1} is: {len}");

    println!("the size of {s1} is: {len}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// referência mutável

fn modifica(uma_string: &mut String) {
    uma_string.push_str("longa");
}

/*Dois ou mais ponteiros acessam o mesmo dado ao mesmo tempo.
Ao menos um dos ponteiros é usado para escrever sobre o dado.
Não há nenhum mecanismo sendo usado para sincronizar o acesso ao dado. */
