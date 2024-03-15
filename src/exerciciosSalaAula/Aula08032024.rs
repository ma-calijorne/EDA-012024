use std::io;
use std::f64::consts::PI;

// Função principal que executa o programa
pub fn principal() {
    println!("Calculadora de área de um círculo");
    println!("Por favor, insira o raio do círculo:");

    let mut raio_str = String::new();
    io::stdin().read_line(&mut raio_str)
        .expect("Falha ao ler entrada");

    let raio: f64 = raio_str.trim().parse()
        .expect("Entrada inválida");

    let area = calcular_area(raio);
    println!("A área do círculo com raio {} é: {}", raio, area);
}

// Função para calcular a área do círculo
fn calcular_area(raio: f64) -> f64 {
    PI * raio.powi(2)
}
