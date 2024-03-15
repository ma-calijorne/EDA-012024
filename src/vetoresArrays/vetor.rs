

/*
Um vetor, por outro lado, é uma coleção dinâmica que pode crescer ou encolher em tempo de execução.
Vetores em Rust são declarados usando a macro vec! ou explicitamente com o tipo Vec<T>.
Aqui está como você pode declará-los:

let nome_do_vetor = vec![valor1, valor2, valor3, ..., valorN];

*/

pub fn declaracaoVetor(){

    let mut numeros = vec![1, 2, 3, 4, 5];
}

/*
Podemos também  especificar o tipo explicitamente:
*/

pub fn declaracaoVetorTipoExplicito(){
    /*Declaramos o vetor
    let nome_do_vetor: Vec<Tipo> = Vec::new();
     E depois adicionamos elementos
    nome_do_vetor.push(valor1);
    nome_do_vetor.push(valor2);
    nome_do_vetor.push(valor3);*/
}

/*
Note que, ao contrário dos arrays, vetores podem mudar de tamanho, então você geralmente
trabalha com eles usando um mut para permitir modificação.
*/


pub fn acessoElementos(){

    /*
        Para acessar elementos de um vetor, você pode usar o índice do elemento, assim como
        faria com um array. Lembre-se de que acessar um índice fora dos limites do
        vetor causará um erro em tempo de execução.
    */
    let numeros = vec![10, 20, 30, 40, 50];
    let primeiro = numeros[0];
    println!("O primeiro número é {}", primeiro);
}

pub fn alterandoValores(){
    /*
    Você pode alterar o valor de um elemento específico em um vetor se o vetor for mutável (mut).
    Você também pode adicionar ou remover elementos do vetor.
    */

    let mut numeros = vec![10, 20, 30, 40, 50];
    // Alterando um valor
    numeros[2] = 35;
    // Adicionando um elemento ao final
    numeros.push(60);
    // Removendo o último elemento
    numeros.pop();

    println!("Números após alterações: {:?}", numeros);

}

pub fn iteracaoElementos(){

    /*
        Para iterar sobre todos os elementos de um vetor, você pode usar um loop for.
        Rust oferece várias maneiras de iterar sobre vetores, incluindo iterar sobre referências
        aos seus elementos ou tomar posse dos elementos.
    */

    let numeros = vec![10, 20, 30, 40, 50];

    // Iterando sobre referências aos elementos
    for numero in &numeros {
        println!("{}", numero);
    }
}


pub fn iteracaoElementosReferenciaMutaveis(){
    /*
    Se você precisar modificar os elementos do vetor enquanto itera,
    você deve iterar sobre referências mutáveis aos elementos:
    */

    let mut numeros = vec![10, 20, 30, 40, 50];

    // Iterando sobre referências mutáveis para modificar elementos
    for numero in &mut numeros {
        *numero += 1; // Incrementa cada número
    }
    println!("Números após incremento: {:?}", numeros);

}

pub fn iteracaoElementosEnumerate(){
    /*
    E para acessar tanto o índice quanto o valor durante a iteração,
    você pode usar o método .enumerate():
    */
    let numeros = vec![10, 20, 30, 40, 50];

    for (i, numero) in numeros.iter().enumerate() {
        println!("Índice: {}, Valor: {}", i, numero);
    }
}
