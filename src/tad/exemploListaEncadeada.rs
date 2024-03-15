/*
Neste exemplo, TaskList representa a lista de tarefas, e TaskNode é a estrutura de cada tarefa na l
ista. Cada TaskNode contém uma String que descreve a tarefa e um Option<Rc<RefCell<TaskNode>>>
que aponta para o próximo nó na lista, permitindo uma estrutura encadeada.
Usamos Rc e RefCell para ter contagem de referência e mutabilidade interior, respectivamente,
o que é útil em contextos onde precisamos de múltiplas referências a um objeto e queremos modificar
esse objeto em tempo de execução.
A função add_task adiciona uma nova tarefa no início da lista para manter a
operação eficiente, evitando a necessidade de percorrer toda a lista para adicionar
um novo item no final. A função print_tasks percorre a lista a partir do início (head) e
imprime cada tarefa.
*/

use std::rc::Rc;
use std::cell::RefCell;

// Definição do nó da lista encadeada
#[derive(Clone)]
struct TaskNode {
    task: String,
    next: Option<Rc<RefCell<TaskNode>>>,
}

// Definição da lista de tarefas
struct TaskList {
    head: Option<Rc<RefCell<TaskNode>>>,
}

impl TaskList {
    fn new() -> Self {
        TaskList { head: None }
    }

    // Adiciona uma nova tarefa ao início da lista
    fn add_task(&mut self, task: String) {
        let new_node = Rc::new(RefCell::new(TaskNode {
            task,
            next: self.head.clone(),
        }));
        self.head = Some(new_node);
    }

    // Imprime todas as tarefas da lista
    fn print_tasks(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            let borrowed_node = node.borrow();
            println!("{}", borrowed_node.task);
            current = borrowed_node.next.clone();
        }
    }
}

pub fn listaEncadeada() {

    let mut my_tasks = TaskList::new();

    my_tasks.add_task("Finalizar relatório".to_string());
    my_tasks.add_task("Reunião com a equipe às 14h".to_string());
    my_tasks.add_task("Enviar e-mail de atualização do projeto".to_string());

    println!("Tarefas pendentes:");
    my_tasks.print_tasks();
}
