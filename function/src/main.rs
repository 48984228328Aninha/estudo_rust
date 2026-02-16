fn main() {
    println!("Hello, world!");

    outra_funcao(5);
}

fn outra_funcao(x: u8) {
    let check = x.checked_add(1);
    
    let y = {
        let x = 3;
        x+1
        //se fosse com ponto e vírgula seria uma instrução e nn teria o valor esperado
        //obgg compilador do rust
    };

    println!("o resultado de y eh: {y}");
    

    match check {
        Some(valor) => println!("O resultado é {}", valor),
        None => println!("erro"),
    }
    println!("retorna isso aq ó: {x}");

    fn somar(x:u8, y:u8) -> u8 {
        let soma = x + y;
        return soma;
    }

    println!("o valor da soma eh: {}", somar(5, 3));

    fn plus_one(x:i32) -> i32 {
        x + 1
    }

    println!("só queria usar o plus_one msm: {}", plus_one(4));
}
