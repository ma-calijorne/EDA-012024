use std::mem;


pub fn rs_types() {
    // Variaveis podem ter a definição do tipo. Fortemente tipadas.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Definição de tipo tradicional
    let an_integer   = 5i32; // definição de tipo com sufixo

    // Ou um padrão será usado.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // Um tipo também pode ser inferido do contexto.
    let mut inferred_type = 12; // O tipo i64 é inferido
    inferred_type = 4294967296i64;

    // O valor de uma variável mutável pode ser alterado.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Erro! O tipo de uma variável não pode ser alterado.
    // mutable = true;

    // As variáveis podem ser substituídas com shadowing.
    let mutable = true;
}


pub fn typeLiteraisOperadores() {
// Literais e operadores
// Inteiros 1, flutuantes 1.2, caracteres 'a', strings "abc", booleanos verdadeiros e o tipo
// de unidade () podem ser expressos usando literais.
// Os inteiros podem, alternativamente, ser expressos usando notação hexadecimal, octal
// ou binária usando estes prefixos respectivamente: 0x, 0o ou 0b.
// Os sublinhados podem ser inseridos em literais numéricos para melhorar a legibilidade,
// por exemplo. 1_000 é igual a 1000 e 0,000_001 é igual a 0,000001.
// Rust também suporta notação E científica, por ex. 1e6, 7.6e-4. O tipo associado é f64.
// Precisamos informar ao compilador o tipo de literais que usamos.
// Por enquanto, usaremos o sufixo u32 para indicar que o literal é um número inteiro
// não assinado de 32 bits e o sufixo i32 para indicar que é um número inteiro assinado de 32 bits.
// Os operadores disponíveis e sua precedência em Rust são semelhantes a outras
// linguagens semelhantes a C.

    // Adição de Inteiro
    println!("1 + 2 = {}", 1u32 + 2);

    // Subtração de inteiro
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Tente mudar `1i32` para `1u32` para ver por que o tipo é importante

    // Notação científica
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Lógica booleana de curto-circuito
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Operações bit a bit
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores para melhorar a legibilidade!
    println!("One million is written as {}", 1_000_000u32);
}


// Esta função pega emprestada uma fatia.
fn analyze_slice(slice: &[i32]) {
    println!("Primeiro elemento da fatia: {}", slice[0]);
    println!("A Fatia tem {} elementos", slice.len());
}

pub fn typeArraysSlices() {
    /*
    Matrizes e fatias
    Um array é uma coleção de objetos do mesmo tipo T, armazenados em memória contígua.
    Arrays são criados usando colchetes [], e seu comprimento, que é conhecido em tempo
    de compilação, faz parte de sua assinatura de tipo [T; comprimento].

    As fatias são semelhantes aos arrays, mas seu comprimento não é conhecido em tempo
    de compilação. Em vez disso, uma fatia é um objeto de duas palavras; a primeira palavra
    é um ponteiro para os dados, a segunda palavra é o comprimento da fatia. O tamanho
    da palavra é igual a usize, determinado pela arquitetura do processador, por ex. 64
    bits em um x86-64. As fatias podem ser usadas para emprestar uma seção de um array
    e ter a assinatura de tipo &[T].
    */

    // Matriz de tamanho fixo (assinatura de tipo é supérflua).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // Todos os elementos podem ser inicializados com o mesmo valor.
    let ys: [i32; 500] = [0; 500];

    //A indexação começa em 0.
    println!("Primeiro Elemento do Array: {}", xs[0]);
    println!("Segundo Elemento do Array: {}", xs[1]);

    // `len` retorna a contagem de elementos no array.
    println!("Número de elementos no array: {}", xs.len());

    // As matrizes são alocadas em pilha.
    println!("Array está ocupando {} bytes", mem::size_of_val(&xs));

    // Matrizes podem ser emprestadas automaticamente como fatias.
    println!("Pegue emprestado todo o array como uma fatia.");
    analyze_slice(&xs);

    // As fatias podem apontar para uma seção de um array.
    // Eles têm o formato [starting_index..ending_index].
    // `starting_index` é a primeira posição na fatia.
    // `ending_index` é um a mais que a última posição na fatia.
    println!("Pegue emprestada uma seção da matriz como uma fatia.");
    analyze_slice(&ys[1 .. 4]);

    // Exemplo de uma fatia vazia `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);

    // Arrays podem ser acessados com segurança usando `.get`, que retorna um
    // `Opção`. Isso pode ser combinado conforme mostrado abaixo ou usado com
    // `.expect()` se você quiser que o programa saia com um bom
    // mensagem em vez de continuar .
    for i in 0..xs.len() + 1 { // Ops, um elemento longe demais!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Calma lá! {} está muito longe!", i),
        }
    }

    // A indexação fora do limite no array causa erro no tempo de compilação.
    //println!("{}", xs[5]);
    // A indexação fora do limite na fatia causa erro de tempo de execução.
    //println!("{}", xs[..][5]);
}


// Globais são declaradas fora de outros escopos.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

pub fn typesContants() {

    /*
    Rust possui dois tipos diferentes de constantes que podem ser declaradas em qualquer escopo,
    inclusive global. Ambos requerem anotação de tipo explícita:
    const: um valor imutável (o caso comum).
    static: Uma variável possivelmente mutável com 'tempo de vida estático. O tempo de vida
    estático é inferido e não precisa ser especificado. Acessar ou modificar uma
    variável estática mutável não é seguro.
    */
    let n = 16;

    // Constante de acesso no thread principal
    println!("This is {}", LANGUAGE);
    println!("O threshold é {}", THRESHOLD);
    println!("{} é {}", n, if is_big(n) { "big" } else { "small" });

    // Erro! Não é possível modificar um `const`.
    //THRESHOLD = 5;
}