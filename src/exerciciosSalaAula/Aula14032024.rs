
/***************************************************************************************************
*                                                                                                  *
*                                        ARRAYS                                                    *
*                                                                                                  *
***************************************************************************************************/

/***************************************************************************************************
Exercício 1: Encontrar o Máximo
Enunciado: Escreva um programa em Rust que encontre o valor máximo em um array de inteiros.
***************************************************************************************************/

pub fn encontraMaximo(){

    let numeros = [10, 20, 5, 23, 42, 15];
    let mut maximo = numeros[0];

    for &numero in &numeros[1..] {
        if numero > maximo {
            maximo = numero;
        }
    }

    println!("O valor máximo é {}", maximo);
}

/***************************************************************************************************
Exercício 2: Reverter um Array
Enunciado: Escreva um programa em Rust que reverta um array de inteiros sem usar métodos prontos da
linguagem para isso.
***************************************************************************************************/

pub fn reverterArray(){

    let mut numeros = [1, 2, 3, 4, 5];
    let tamanho = numeros.len();
    for i in 0..tamanho / 2 {
        numeros.swap(i, tamanho - 1 - i);
    }

    println!("Array revertido: {:?}", numeros);
}

/***************************************************************************************************
Exercício 3: Calcular a Média
Enunciado: Escreva um programa em Rust que calcule a média dos valores armazenados em um
array de números de ponto flutuante.
***************************************************************************************************/

pub fn calcularMedia(){

    let numeros = [10.0, 20.5, 30.0, 40.5, 50.0];

    let soma: f64 = numeros.iter().sum();
    let media = soma / numeros.len() as f64;

    println!("A média é {}", media);
}
/***************************************************************************************************
Exercício 4: Contar Elementos Negativos
Enunciado: Escreva um programa em Rust que conte quantos números negativos existem em um
array de inteiros.
***************************************************************************************************/

pub fn contarNumerosNegativos(){

    let numeros = [10, -20, -5, 23, 42, -15, 0];

    let mut contador_negativos = 0;

    for &numero in &numeros {
        if numero < 0 {
            contador_negativos += 1;
        }
    }

    println!("Existem {} números negativos no array.", contador_negativos);
}

/***************************************************************************************************
Exercício 5: Verificar Presença de Elemento
Enunciado: Escreva um programa em Rust que verifique se um determinado inteiro está
presente em um array.
***************************************************************************************************/

pub fn verificarPresencaElemento(){

    let numeros = [1, 2, 3, 4, 5];
    let alvo = 3;
    let mut encontrado = false;

    for &numero in &numeros {
        if numero == alvo {
            encontrado = true;
            break;
        }
    }

    if encontrado {
        println!("O número {} está presente no array.", alvo);
    } else {
        println!("O número {} não foi encontrado no array.", alvo);
    }
}

/***************************************************************************************************
*                                                                                                  *
*                                        VETORES                                                   *
*                                                                                                  *
***************************************************************************************************/

/***************************************************************************************************
Exercício 1: Adicionar Elementos a um Vetor
Enunciado: Escreva um programa em Rust que crie um vetor vazio e adicione a ele os
números de 1 a 5, um de cada vez, usando um loop.
***************************************************************************************************/

pub fn adicionarElemento(){

    let mut numeros = Vec::new();

    for i in 1..=5 {
        numeros.push(i);
    }

    println!("Vetor: {:?}", numeros);
}

/***************************************************************************************************
Exercício 2: Remover Elemento Específico
Enunciado: Escreva um programa em Rust que remova o primeiro elemento de
valor 3 de um vetor de inteiros.
***************************************************************************************************/

pub fn removerElementoEspecifico(){

    let mut numeros = vec![1, 2, 3, 4, 3, 5];

    if let Some(pos) = numeros.iter().position(|&x| x == 3) {
        numeros.remove(pos);
    }

    println!("Vetor após remover o primeiro '3': {:?}", numeros);
}

/***************************************************************************************************
Exercício 3: Calcular a Soma de Elementos
Enunciado: Escreva um programa em Rust que calcule a soma de todos os elementos
em um vetor de números inteiros.
***************************************************************************************************/

pub fn calcularSomaElementos(){

    let numeros = vec![10, 20, 30, 40, 50];
    let soma: i32 = numeros.iter().sum();

    println!("A soma dos elementos é {}", soma);
}

/***************************************************************************************************
Exercício 4: Encontrar o Menor Elemento
Enunciado: Escreva um programa em Rust que encontre o menor elemento em um vetor de números
inteiros.
***************************************************************************************************/

pub fn encontrarMenorNumero(){

    let numeros = vec![24, 42, 12, 8, 15];
    let mut menor = numeros[0];

    for &valor in &numeros {
        if valor < menor {
            menor = valor;
        }
    }

    println!("O menor número é {}", menor);
}
//Código Resposta

/***************************************************************************************************
Exercício 5: Filtrar Elementos Pares e Criar um Novo Vetor
Enunciado: Escreva um programa em Rust que, dado um vetor de números inteiros, crie
um novo vetor contendo apenas os elementos pares do vetor original.
***************************************************************************************************/

pub fn filtrarELementosPares(){

    let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let pares: Vec<i32> = numeros.into_iter().filter(|x| x % 2 == 0).collect();

    println!("Números pares: {:?}", pares);
}


/***************************************************************************************************
*                                                                                                  *
*                                        STRUCT                                                    *
*                                                                                                  *
***************************************************************************************************/

/***************************************************************************************************
Exercício 1: Definir e Instanciar uma Struct
Enunciado: Defina uma struct Carro que tenha três campos: marca, modelo, e ano.
Crie uma instância dessa struct e imprima seus valores no console.
***************************************************************************************************/

// struct Carro {
//     marca: String,
//     modelo: String,
//     ano: u16,
// }
//
// pub fn definirInstanciaStruct() {
//     let meu_carro = Carro {
//         marca: String::from("Toyota"),
//         modelo: String::from("Corolla"),
//         ano: 2020,
//     };
//
//     println!("Marca: {}, Modelo: {}, Ano: {}", meu_carro.marca, meu_carro.modelo, meu_carro.ano);
// }

/***************************************************************************************************
Exercício 2: Adicionar Método à Struct
Enunciado: Utilizando a struct Carro do exercício anterior, adicione um método descricao que
retorna uma string formatada com todos os dados do carro. Chame este método para uma
instância de Carro e imprima o resultado.
***************************************************************************************************/

struct Carro {
    marca: String,
    modelo: String,
    ano: u16,
}

impl Carro {
    fn descricao(&self) -> String {

        format!("{} {} {}", self.marca, self.modelo, self.ano)
    }
}

pub fn adicionarMetodoStruct() {
    let meu_carro = Carro {
        marca: String::from("Toyota"),
        modelo: String::from("Corolla"),
        ano: 2020,
    };

    println!("{}", meu_carro.descricao());
}

/***************************************************************************************************
Exercício 3: Struct com Enum
Enunciado: Crie uma struct Pedido que contém nome_do_item (String) e status (um enum StatusPedido
com variantes Pendente, Concluido). Adicione um método à struct Pedido que imprime uma
mensagem diferente dependendo do status do pedido.
***************************************************************************************************/

struct Pedido {
    nome_do_item: String,
    status: StatusPedido,
}

enum StatusPedido {
    Pendente,
    Concluido,
}

impl Pedido {
    fn verificar_status(&self) {
        match self.status {
            StatusPedido::Pendente => println!("{} está pendente.", self.nome_do_item),
            StatusPedido::Concluido => println!("{} foi concluído!", self.nome_do_item),
        }
    }
}

pub fn structComEnum() {
    let meu_pedido = Pedido {
        nome_do_item: String::from("Livro Rust"),
        status: StatusPedido::Concluido,
    };

    meu_pedido.verificar_status();
}

/***************************************************************************************************
Exercício 4: Struct com Lifetimes
Enunciado: Defina uma struct Livro que tenha dois campos: titulo e autor, ambos
sendo referências a strings com um lifetime específico. Crie uma instância dessa struct e
uma função que aceita uma referência a Livro e imprime o título e o autor.
***************************************************************************************************/

struct Livro<'a> {
    titulo: &'a str,
    autor: &'a str,
}

fn exibir_livro(livro: &Livro) {
    println!("'{}' por {}", livro.titulo, livro.autor);
}

pub fn lifetimes() {
    let livro_rust = Livro {
        titulo: "The Rust Programming Language",
        autor: "Steve Klabnik and Carol Nichols",
    };

    exibir_livro(&livro_rust);
}

/***************************************************************************************************
Exercício 5: Struct com Vários Métodos
Enunciado: Crie uma struct Contador com um campo valor do tipo i32. Adicione métodos para
incrementar e decrementar o valor, além de um método que retorna o valor atual. Demonstre o uso
dessa struct incrementando, decrementando o valor e exibindo o valor atual.
***************************************************************************************************/

struct Contador {
    valor: i32,
}

impl Contador {
    fn incrementar(&mut self) {
        self.valor += 1;
    }

    fn decrementar(&mut self) {
        self.valor -= 1;
    }

    fn valor_atual(&self) -> i32 {
        self.valor
    }
}

pub fn structVariosMetodos() {
    let mut meu_contador = Contador { valor: 0 };

    meu_contador.incrementar();
    meu_contador.incrementar();
    meu_contador.decrementar();

    println!("Valor atual: {}", meu_contador.valor_atual());
}

