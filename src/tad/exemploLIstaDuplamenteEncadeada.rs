/*
Este exemplo ilustra uma maneira básica de como uma lista duplamente encadeada pode ser
utilizada para criar um sistema de histórico de navegação num navegador web,
permitindo ao usuário avançar e retroceder pelo histórico de páginas visitadas.
*/
use std::rc::Rc;
use std::cell::RefCell;

// Definição do nó da lista duplamente encadeada
struct PageNode {
    url: String,
    prev: Option<Rc<RefCell<PageNode>>>,
    next: Option<Rc<RefCell<PageNode>>>,
}

// Definição do histórico de navegação
struct BrowserHistory {
    current: Option<Rc<RefCell<PageNode>>>,
}

impl BrowserHistory {
    fn new() -> Self {
        BrowserHistory { current: None }
    }

    // Adiciona uma nova página ao histórico e navega para ela
    fn visit_page(&mut self, url: String) {
        let new_page = Rc::new(RefCell::new(PageNode {
            url,
            prev: self.current.clone(),
            next: None,
        }));

        if let Some(current_page) = &self.current {
            current_page.borrow_mut().next = Some(new_page.clone());
        }

        self.current = Some(new_page);
    }

    // Navega para a página anterior, se possível
    fn go_back(&mut self) {
        if let Some(current_page) = &self.current.take() { // Use take() para temporariamente remover `current` do estado, evitando empréstimos mutáveis
            self.current = current_page.borrow().prev.clone();
        }
    }

    // Navega para a próxima página, se possível
    fn go_forward(&mut self) {
        if let Some(current_page) = &self.current.take() { // Similarmente, use take() aqui
            self.current = current_page.borrow().next.clone();
        }
    }

    // Imprime a URL da página atual
    fn print_current_url(&self) {
        if let Some(current_page) = &self.current {
            println!("URL Atual: {}", current_page.borrow().url);
        } else {
            println!("Nenhuma página sendo exibida.");
        }
    }
}

pub fn listDuplamenteEncadeada() {
    let mut history = BrowserHistory::new();

    history.visit_page("https://www.example.com".to_string());
    history.visit_page("https://www.google.com".to_string());
    history.visit_page("https://www.rust-lang.org".to_string());

    history.print_current_url(); // Imprime a URL atual
    history.go_back();
    history.print_current_url(); // Imprime a URL da página anterior
    history.go_forward();
    history.print_current_url(); // Volta para a URL atual
}
