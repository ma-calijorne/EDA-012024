/*
Este código Rust define uma função simple_hash que implementa nossa função hash matemática,
tomando um i32 como entrada e retornando o valor de x % 10, que é o índice hash do número.
No main, criamos um vetor de números inteiros para demonstrar como diferentes números são
hashados para índices de 0 a 9 usando essa função hash simples.
*/

fn simple_hash(x: i32) -> i32 {
    x % 10
}

pub fn simpleHashFunction() {
    let numbers = vec![11, 22, 33, 44, 123, 999, 1000];

    for number in numbers {
        let hash = simple_hash(number);
        println!("hash({}) = {}", number, hash);
    }
}