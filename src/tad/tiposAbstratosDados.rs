use std::collections::LinkedList;

pub fn tadString() {


    //Este código cria uma string mutável s que contém o texto "Hello, world!".
    let mut s = String::from("Algorítmos e Estruturas de Dados");

    //push_str(&str): Acrescenta uma string literal ao final de uma String

    s.push_str("- Primeiro Semestre 2024");

    //push(char): Acrescenta um caractere ao final de uma String.
    s.push('d');

    //replace(&str, &str): Substitui todas as ocorrências de um padrão dentro da String por outra string.

    let new_s = s.replace("Primeiro", "Segundo");

    //len(): Retorna o comprimento da String em bytes, não em caracteres Unicode.
    println!("O tamanho da string s é {}",s.len());
}

pub fn tadArray() {
    /*
    Este exemplo declara um array de strings que representam os dias da semana.
    O array é imutável por padrão, e tentar alterar um de seus elementos resultará em um erro
    de compilação, a menos que o array seja declarado como mutável usando mut.
    */
    let dias_da_semana: [&str; 7] = [
        "Domingo", "Segunda", "Terça", "Quarta", "Quinta", "Sexta", "Sábado"
    ];

    println!("O primeiro dia da semana é: {}", dias_da_semana[0]);
}


pub fn tadVector() {
    /*
    Este exemplo cria um vetor de inteiros, adiciona três números a ele e depois remove o
    último elemento. O método push é usado para adicionar elementos ao final do vetor,
    enquanto pop remove e retorna o último elemento. O método len retorna o número de
     elementos atualmente no vetor, ilustrando a flexibilidade dos vetores em comparação
     com arrays de tamanho fixo.
    */

    let mut numeros: Vec<i32> = Vec::new();
    numeros.push(1);
    numeros.push(2);
    numeros.push(3);

    println!("O primeiro número é: {}", numeros[0]);

    // Removendo o último elemento
    numeros.pop();
    println!("Após remover, o vetor tem {} elementos.", numeros.len());
}

pub fn tadVector_2() {
    /*
    Neste exemplo, um vetor de strings é criado usando a macro vec!, inicializado com "Maçã" e
    "Banana". Em seguida, "Laranja" é adicionada ao vetor usando o método push.
    O loop for é usado para iterar sobre o vetor e imprimir cada elemento.
    */
    let mut frutas = vec!["Maçã", "Banana"];
    frutas.push("Laranja"); // Adiciona "Laranja" ao final do vetor

    println!("Frutas disponíveis:");
    for fruta in &frutas {
        println!("{}", fruta);
    }
}

pub fn tadVector_3() {
    /*
        Neste segundo exemplo, um vetor de inteiros é criado e o último elemento é removido usando
        o método `pop`, que reduz o tamanho do vetor em uma unidade. Para acessar um elemento
        sem correr o risco de um panic por acessar um índice inexistente, utiliza-se o
        método `get`, que retorna `Some(&T)` se o elemento existir ou `None` se o
        índice estiver fora dos limites do vetor. O acesso direto `[1]` é demonstrado para o
        segundo elemento, mas deve ser usado com cuidado, pois pode causar
        um panic se o índice especificado não estiver dentro do intervalo válido do vetor.
    */
    let mut numeros = vec![1, 2, 3, 4, 5];
    numeros.pop(); // Remove o último elemento, neste caso, o número 5

    if let Some(primeiro_numero) = numeros.get(0) {
        println!("O primeiro número é: {}", primeiro_numero);
    } else {
        println!("O vetor está vazio.");
    }

    // Acessando diretamente pelo índice, o que pode causar panic se o índice não existir
    println!("O segundo número é: {}", numeros[1]);
}




pub fn tadList() {
    /*Este exemplo demonstra o uso da LinkedList da biblioteca padrão do Rust,
    incluindo a adição de elementos ao início e ao fim da lista, a remoção de elementos e a
    travessia da lista para acessar seus elementos.
     */
    let mut lista = LinkedList::new();

    // Adicionando elementos à lista
    lista.push_back("Elemento 1");
    lista.push_front("Elemento 2");

    // Removendo um elemento
    lista.pop_back();

    // Acessando elementos (travessia)
    for elemento in lista.iter() {
        println!("{}", elemento);
    }
}




