/*
Neste exemplo, a struct Stack é genérica, permitindo que você crie pilhas de qualquer tipo de dados.
 A implementação usa um Vec<T> para armazenar os elementos da pilha, aproveitando os métodos que
 o vetor fornece para implementar as operações de push e pop. O método peek permite olhar o
 elemento no topo da pilha sem removê-lo, e is_empty verifica se a pilha está vazia.
 Este exemplo ilustra como as características fundamentais das pilhas são aplicadas em Rust,
 oferecendo uma estrutura de dados flexível e útil para várias aplicações.
*/

struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    // Cria uma nova pilha vazia
    fn new() -> Self {
        Stack { elements: Vec::new() }
    }

    // Adiciona um elemento ao topo da pilha
    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    // Remove e retorna o elemento do topo da pilha, se houver algum
    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // Retorna uma referência ao elemento no topo da pilha, sem removê-lo
    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    // Verifica se a pilha está vazia
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

pub fn tadPilha() {
    let mut stack = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Topo da pilha: {:?}", stack.peek()); // Deve imprimir 3

    while let Some(top) = stack.pop() {
        println!("Removido: {}", top);
    }

    println!("A pilha está vazia? {}", stack.is_empty()); // Deve imprimir true
}
