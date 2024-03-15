

/*Um array em Rust tem tamanho fixo e todos os elementos devem ser do mesmo tipo.
A sintaxe para declarar um array é:

let nome_do_array: [Tipo; Tamanho] = [valor1, valor2, valor3, ..., valorN];

*/

pub fn declaracaoArray(){
    let numeros: [i32; 5] = [1, 2, 3, 4, 5];
}

pub fn acessoElementos(){

    /*
        Podemos acessar elementos de um array usando índices, que começam em 0.
        A tentativa de acessar um índice fora dos limites do array causará um erro em
        tempo de execução.
    */
    let numeros = [10, 20, 30, 40, 50];
    let primeiro = numeros[0];
    println!("O primeiro número é {}", primeiro);
}

pub fn alterandoValores(){
    /*
    Para alterar um valor em um array, devemos primeiro tornar o array mutável usando a
    palavra-chave mut. Depois, você pode atribuir um novo valor a um índice específico.
    */

    let mut numeros = [10, 20, 30, 40, 50];
    numeros[2] = 35;
    println!("Números após a alteração: {:?}", numeros);

}

pub fn iteracaoElementos(){

    /*
    Podemos iterar sobre os elementos de um array usando um loop for.
    Isso permite que você acesse cada elemento do array sequencialmente.
    */

    let numeros = [10, 20, 30, 40, 50];

    for numero in numeros.iter() {
        println!("{}", numero);
    }
}

pub fn iteracaoElementosEnumerate(){
    /*
    Além disso, se for preciso de acesso ao índice dos elementos durante a iteração,
    podemos usar o método .enumerate(), que adiciona um contador aos itens do iterador:
    */
    let numeros = [10, 20, 30, 40, 50];

    for (i, numero) in numeros.iter().enumerate() {
        println!("Índice: {}, Valor: {}", i, numero);
    }
}

