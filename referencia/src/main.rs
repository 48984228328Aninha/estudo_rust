fn main() {
    let s1 = String::from("Ola");
    let len = calculate_length(&s1);

    let s2 = String::from("iai :)")
    let len2 = calculate_length(&s2);

    let mut s = String::from("hello");

    let referencia_para_o_nada = soltar();

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

fn soltar() -> String {
    let s = String::from("txt");

    s; //tem que ser sem a referência, se não ela deixa de existir depois do escopo
    //da função

    //só é possível mudar o valor da referência de uma variável se ela for mutável, 
    //o mesmo vale para referência
}

/* condições de data race:
Dois ou mais ponteiros acessam o mesmo dado ao mesmo tempo.
Ao menos um dos ponteiros é usado para escrever sobre o dado.
Não há nenhum mecanismo sendo usado para sincronizar o acesso ao dado. */

//pode:

let r1 = &s;
let r2 = &s;

//não pode >:(
/*
    let r1 = &mut s;
 */


