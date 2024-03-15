use std::io;
pub fn calculoAreaCirculo() {

    println!("Calculadora de Área de Círculo");
    println!("Por favor, insira o valor do raio do círculo:");
    let mut raio = String::new();

    io::stdin().read_line(&mut raio)
        .expect("Falha ao ler a entrada");

    let raio: f64 = raio.trim().parse()
        .expect("Por favor, insira um número válido");

    let area = calcular_area(raio);

    println!("A área do círculo com raio {} é {:.2}", raio, area);
}

fn calcular_area(raio: f64) -> f64 {

    const PI: f64 = std::f64::consts::PI;
    PI * raio.powi(2)
}
