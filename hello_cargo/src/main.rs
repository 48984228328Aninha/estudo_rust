fn main() {
    //usando wrapping p evitar int overflow
    let x:u8 = 255;

    let result = x.wrapping_add(1);
    println!("{}", result);

        let resultado = x.wrapping_mul(2);
        println!["{}", resultado];

        println!("{x}");

    //agr vou usar checked, o + seguro

    let y:u8 = 255;
    let checar = y.checked_add(1);

    match checar {
        Some(valor) => println!("Resultado: {}", valor),
        None => println!("Erro"),
    }


}