<div align=center>
<h1>Estudos Rust <3</h1>
<p>Um repositório feito para estudar Rust, documentando informações que acho importante de serem mencionadas e que fazem a linguagem ser diferente de um jeito bom.</p>
</div>

  <div>
  <h3>Serve para</h3>
  <ul>
    <li>Consultar todos os conceitos da linguagem dentro das pastas</li>
    <li>Guia de estudo com todos os códigos da documentação oficial (O livro)</li>
  </ul>
  </div>

<div align=center>
<h1>Stack e Heap: diferença em Rust</h1>
</div>

# Stack
- Usada para guardar valores e acessar o último valor, parecido com uma pilha de pratos, adiciona-se pratos e pega-se o último. Adicionar algo no meio ou nos primeiros pode se tornar difícil. Todos eles devem ter tamanho conhecido

# Heap
- Alocador de memória encontra um espaço vazio e retorna um ponteiro

# Comparação
- Colocar na pilha é mais rápido, já que será adicionado ao último elemento da stack. Acessar dados no heap é mais lento

# Escopo:
{
let c = "hello";
}

# Uma string pode ser escrita assim:
let mut s = String::from("hello);
s.push_str(", world");

println"({s});

# Como uma string é armazenada?
<img src="https://doc.rust-lang.org/stable/book/img/trpl04-01.svg">

# Rust não copia dados fielmente, usa apenas referência ou movimento para economizar memória. Para copiar fielmente, use clone.

let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");
