fn main() {
    println!("Hello, world!");

    outra_funcao(5);
}

fn outra_funcao(x: u8) {
    let check = x.checked_add(1);
    
    let y = {
        let x = 3;
        x+1
    };

    println!("o resultado de y eh: {y}");
    

    match check {
        Some(valor) => println!("O resultado é {}", valor),
        None => println!("erro"),
    }
    println!("retorna isso aq ó: {x}");
}
