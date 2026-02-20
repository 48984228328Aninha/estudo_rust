fn main() {
    let number: u8 = 254;

    let checar = number.checked_add(1);

    match checar {
        Some(number) => println!("Resultado: {}", number),
        None => println!("Erro"),
    }

    if number == 254 {
        println!("é igual");
    } else {
        println!("não é igual");
    }

    //usando if em uma instrução let
    let condition = true;
    let check = if condition {254} else {0};

    println!("The value of number is: {number}, {check}");

    //loop executa várias vezes até eu mandar parar
    /*loop {
        println!("loop");
    }*/

    let mut counter= 0;
    let result = loop {
        counter+=1;

        if counter == 0 {
            break counter * 2;
        }
    };

    println!("o valor é: {counter} e {result}");
}
