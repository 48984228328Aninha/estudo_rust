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


}