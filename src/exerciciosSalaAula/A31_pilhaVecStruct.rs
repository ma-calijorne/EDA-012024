use std::io;

// Definindo a struct Processo que representa cada processo.
#[derive(Debug)]
struct Processo {
    pid: i32,
    men_size: i32,
    time_execution: i32,
}

// Definindo a struct Pilha que utilizará um Vector para armazenar os processos.
struct Pilha {
    processos: Vec<Processo>,
    next_pid: i32,
}

impl Pilha {
    // Método para criar uma nova Pilha vazia.
    fn new() -> Pilha {
        Pilha {
            processos: Vec::new(),
            next_pid: 0,
        }
    }

    // Método para adicionar um novo processo à Pilha.
    fn push(&mut self, men_size: i32, time_execution: i32) {
        let processo = Processo {
            pid: self.next_pid,
            men_size,
            time_execution,
        };
        self.processos.push(processo);
        self.next_pid += 1;
    }

    // Método para imprimir os PIDs dos processos na ordem de inserção.
    fn print_pids(&self) {
        println!("PIDs dos processos na pilha:");
        for processo in &self.processos {
            println!("{}", processo.pid);
        }
    }
}

pub fn mainPilha() {
    let mut pilha = Pilha::new();

    loop {
        println!("Digite o tamanho de memória e o tempo de execução do processo, ou 'sair' para finalizar:");

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Falha ao ler a linha");

        if entrada.trim() == "sair" {
            break;
        }

        let partes: Vec<&str> = entrada.trim().split_whitespace().collect();
        if partes.len() != 2 {
            println!("Entrada inválida. Por favor, forneça dois números.");
            continue;
        }

        let men_size: i32 = partes[0].parse().expect("Falha ao parsear o tamanho de memória");
        let time_execution: i32 = partes[1].parse().expect("Falha ao parsear o tempo de execução");

        if time_execution < 30 || time_execution > 90 {
            println!("O tempo de execução deve estar entre 30 e 90 segundos.");
            continue;
        }

        pilha.push(men_size, time_execution);
    }

    pilha.print_pids();
}
