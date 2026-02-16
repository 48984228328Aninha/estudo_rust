use std::u8;

fn main() {
    //usando wrapping p evitar int overflow, ele começa dnv no 0
    let x:u8 = 255;

    let result = x.wrapping_add(1);
    println!("{}", result);

        let resultado = x.wrapping_mul(2);
        println!["{}", resultado];

        println!("{x}");

    //agr vou usar checked, o + seguro, mostra o erro

    let y:u8 = 255;
    let checar = y.checked_add(1);

    match checar {
        Some(valor) => println!("Resultado: {}", valor),
        None => println!("Erro"),
    }

    //overflowing devolve valor + aviso

    let z:u8 = 255;
    let (resultado, overflow) = x.overflowing_add(1);

    println!("Resultado: {}", resultado);
    println!("Overflow: {}", overflow);

    //saturating para no máx.
    let l:u8 = 255;
    let resultado = x.saturating_add(100);

    println!("resultado: {}", resultado);

    //para criar uma tupla de cria
    let tup: (u8, i8, u8) = (10, -3, 9);
    let (x,y,z) = tup;
    println!("O valor de y da tupla de cria é: {y}");

    //dá para fazer assim tbm
    let first_element = tup.0;
    let second_element = tup.1;
    let third_element = tup.2;

    println!("{first_element}, {second_element}, {third_element}");

    //acesso a elementos de um array de cria
    let arr: [u8; 5] = [1,2,3,4,5];
    let first:u8 = arr[0];
    let second:u8 = arr[1];

    println!("{first}, {second}");

}