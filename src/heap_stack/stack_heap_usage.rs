use std::sync::Arc;
use std::thread;


pub fn stackHeapUsage() {
    // Variável na stack
    let x: i32 = 10;
    println!("Valor na stack: {}", x);

    // Variável na heap
    let y: String = String::from("Olá, mundo!");
    println!("Valor na heap: {}", y);

    // Copiando valores da stack
    let x_copia = x; // Copia o valor de x (tipos simples são armazenados na stack)
    println!("Cópia do valor na stack: {}", x_copia);

    // Movendo valores da heap
    let y_movido = y; // Move o valor de y. `y` não é mais válido após esta linha
    // println!("Valor na heap: {}", y); // Isso geraria um erro de compilação
    println!("Valor movido da heap: {}", y_movido);

    // Clonando valores da heap (para evitar mover)
    let y_clonado = y_movido.clone(); // Cria uma cópia do valor na heap
    println!("Valor clonado da heap: {}", y_clonado);
}


pub fn stackHeapDeclaracaoVariaveis() {
    /*
    Variáveis na Stack: A variável x é um exemplo de dado armazenado na stack.
    É rápido e eficiente, mas seu valor e tamanho são fixos após a declaração.
    Variáveis na Heap: O Box::new(42) aloca um inteiro na heap. A heap é usada para
    dados cujo tamanho pode mudar ou não é conhecido em tempo de compilação.
    */

    // Variável na stack
    let x = 42; // i32 é um tipo de dados que é armazenado na stack
    println!("Valor na stack: {}", x);

    // Variável na heap
    let y = Box::new(42); // Box aloca espaço na heap para um i32
    println!("Valor na heap: {}", *y);

    // Demonstração de propriedade e movimento
    let s1 = String::from("Olá, mundo!"); // String é armazenada na heap
    let s2 = s1; // s1 é movido para s2, s1 não é mais válido

    // println!("{}", s1); // Isso causaria um erro de compilação porque s1 foi movido para s2
    println!("s2 possui o valor de s1: {}", s2);

    // Empréstimo
    let len = calculate_length(&s2); // s2 é emprestado, não movido
    println!("O comprimento de '{}' é {}.", s2, len);

    // Mutabilidade em empréstimos
    let mut s3 = String::from("Olá");
    append_world(&mut s3); // Passa s3 como uma referência mutável
    println!("s3 após modificação: {}", s3);
}

fn calculate_length(s: &String) -> usize { // s é uma referência a uma String
    s.len()
}

fn append_world(s: &mut String) { // s é uma referência mutável a uma String
    s.push_str(", mundo!");
}


pub fn detalhamentoAlocacaoHeap(){

    /*
    Box é um tipo de ponteiro inteligente que aloca dados na heap. Quando um Box é descartado
    (quando sai de escopo, por exemplo), a memória na heap que ele aponta é desalocada.
    Isso é feito automaticamente, graças ao sistema de propriedade de Rust,
    eliminando muitos erros comuns de gerenciamento de memória.
    Neste exemplo, Box::new(10) aloca um espaço na heap para um valor inteiro 10.
    A variável heap_data é um ponteiro para esse espaço na heap. Quando heap_data sai de
    escopo, a memória na heap é automaticamente liberada.
    */

    // Aloca um inteiro na heap
    let heap_data = Box::new(10);
    println!("Valor na heap: {}", heap_data);

    // `heap_data` é automaticamente liberado aqui, quando sai de escopo

    /* Outras Formas de Alocação na Heap
    Além de Box, Rust oferece outros ponteiros inteligentes para gerenciamento de memória na heap, como:

    Rc (Reference Counted): Permite múltiplas propriedades de um valor na heap, contando o
    número de referências para garantir que a memória seja liberada quando não houver mais referências.

    Arc (Atomic Reference Counted): Semelhante ao Rc, mas seguro para uso em contextos de
    concorrência, onde múltiplas threads podem ter propriedade de um valor na heap.

    */

    let valor = Arc::new(5);
    let valor_clone = Arc::clone(&valor);

    let nova_thread = thread::spawn(move || {
        println!("Valor na thread nova: {}", valor_clone);
    });

    nova_thread.join().unwrap();
    println!("Valor na thread principal: {}", valor);

    /*
    Neste exemplo, Arc::clone(&valor) é usado para criar uma cópia da referência que pode ser
    movida para uma nova thread (nova_thread). Isso permite que o dado na heap seja acessado
    de forma segura por múltiplas threads.

    Ambos os exemplos demonstram a alocação de memória na heap e o compartilhamento seguro de dados
    entre múltiplas referências (Rc) ou threads (Arc). A escolha entre Rc e Arc depende das
    necessidades de concorrência do seu programa.
    */
}

