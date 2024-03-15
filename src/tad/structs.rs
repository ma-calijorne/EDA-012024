
/*
Este exemplo mostra uma struct muito simples para representar um ponto em um espaço bidimensional.
Neste exemplo, a struct Ponto é definida com dois campos, x e y, que representam suas coordenadas
em um plano. Um valor da struct é então criado e seus campos são acessados para impressão.
*/
struct Ponto {
    x: i32,
    y: i32,
}

pub fn pontoNoEspaco() {
    let ponto = Ponto { x: 10, y: 20 };
    println!("Ponto está em ({}, {})", ponto.x, ponto.y);
}

/*
Struct com Método
Este exemplo aumenta a complexidade ao adicionar um método à struct, permitindo que
ela tenha comportamentos associados a seus dados.
Aqui, a impl é usada para definir um método distancia_ao_origem para a struct Ponto,
que calcula a distância do ponto até a origem do plano. Isso mostra como você pode encapsular a
funcionalidade relacionada aos dados de uma struct.
*/

impl Ponto {
    fn distancia_ao_origem(&self) -> f64 {
        ((self.x.pow(2) as f64) + (self.y.pow(2) as f64)).sqrt()
    }
}

pub fn structComMetodo() {
    let ponto = Ponto { x: 10, y: 20 };
    println!("Distância ao origem: {}", ponto.distancia_ao_origem());
}


/*
Struct com Lifetimes e Referências
Este exemplo envolve o uso de lifetimes em structs que armazenam referências, demonstrando como Rust
lida com a gestão da memória e o empréstimo de dados.
Neste exemplo, a struct Livro contém referências a strings (&str) para titulo e autor, o que
exige a especificação de um lifetime 'a. Isso garante que as referências dentro de Livro não
sobrevivam àquilo a que elas estão referenciando. A função exibe_informacoes aceita uma
referência a um Livro e imprime suas informações.
*/

struct Livro<'a> {
    titulo: &'a str,
    autor: &'a str,
    ano: i32,
}

fn exibe_informacoes(livro: &Livro) {
    println!("'{}' por {}, {}", livro.titulo, livro.autor, livro.ano);
}

fn main() {
    let livro = Livro {
        titulo: "O Senhor dos Anéis",
        autor: "J.R.R. Tolkien",
        ano: 1954,
    };

    exibe_informacoes(&livro);
}
