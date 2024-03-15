pub fn controleFluxoIfElse() {
// A ramificação com if-else é semelhante a outras linguagens. Ao contrário de muitas delas,
// a condição booleana não precisa estar entre parênteses e cada condição é seguida
// por um bloco. Condicionais if-else são expressões e todas as ramificações
// devem retornar o mesmo tipo.
    let n = 5;

    if n < 0 {
        print!("{} É negativo", n);
    } else if n > 0 {
        print!("{} é Positivo", n);
    } else {
        print!("{} É Zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", e é um número pequeno, aumente dez vezes");

            // Esta expressão retorna um `i32`..
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // Esta expressão também deve retornar um `i32`.
            n / 2
            // TODO ^ Tente suprimir esta expressão com ponto e vírgula
        };

    println!("{} -> {}", n, big_n);
}

pub fn controleFluxoLoop() {
// Rust fornece uma palavra-chave loop para indicar um loop infinito.
// A instrução break pode ser usada para sair de um loop a qualquer momento,
// enquanto a instrução continue pode ser usada para pular o resto da iteração e iniciar uma nova.
    let mut count = 0u32;

    println!("Vamos contar até o infinito!");

    // Loop infinito
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Pular o resto desta iteração
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, já chega");

            // Sair do Loop
            break;
        }
    }
}

pub fn controleFluxoLoopAninhado() {
// É possível interromper ou continuar loops externos ao lidar com loops aninhados. Nestes casos,
// os loops devem ser anotados com algum rótulo ', e o rótulo deve ser passado para a instrução break/continue.
    'outer: loop {
        println!("Entrou no Loop Externo");

        'inner: loop {
            println!("Entrou no loop interno");

            // Isso quebraria apenas o loop interno
            //break;

            // Isso quebra o loop externo
            break 'outer;
        }

        println!("Este ponto nunca será alcançado");
    }

    println!("Saiu do loop externo");
}

pub fn controleFluxoLoopRetorno() {
// Retornando de loops
// Um dos usos de um loop é tentar novamente uma operação até obter sucesso.
// Porém, se a operação retornar um valor, talvez seja necessário passá-lo para o restante
// do código: coloque-o após o intervalo e ele será retornado pela expressão de loop.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

pub fn controleFluxoWhile() {
//While
// A palavra-chave while pode ser usada para executar um loop enquanto uma condição for verdadeira.
// Vamos escrever o infame FizzBuzz usando um loop while.

    // Variável contadora
    let mut n = 1;

    // Loop enquanto n for menor que 101
    while n < 101 {
        if n % 15 == 0 {
            println!("Saida A");
        } else if n % 3 == 0 {
            println!("Saida B");
        } else if n % 5 == 0 {
            println!("Saída C");
        } else {
            println!("{}", n);
        }

        // Incrementar o contador
        n += 1;
    }
}


pub fn controleFluxofor() {

    /*Loops for
    A construção for in pode ser usada para iterar por meio de um Iterador.
    Uma das maneiras mais fáceis de criar um iterador é usar a notação de
    intervalo a..b. Isso produz valores de a (inclusivo) a b (exclusivo) em etapas de um.
    Vamos escrever o um código usando for em vez de while.
    */

    // `n` assumirá valores como :1, 2, ..., 100 em cada uma das interações
    for n in 1..101 {
    // `n` assumirá valores como :1, 2, ..., 100 em cada uma das interações
    // for n in 1..=100 {
        if n % 15 == 0 {
            println!("Saída A");
        } else if n % 3 == 0 {
            println!("Saída B");
        } else if n % 5 == 0 {
            println!("Saída C");
        } else {
            println!("{}", n);
        }
    }
}

pub fn controleFluxoForInteradores_iter() {
    /*
    A construção for in é capaz de interagir com um Iterador de diversas maneiras.
    into_iter, iter e iter_mut lidam com a conversão de uma coleção em um iterador de
    maneiras diferentes, fornecendo diferentes visualizações dos dados contidos nela.

    iter - Pega emprestado cada elemento da coleção em cada iteração.
    Deixando assim a coleção intocada e disponível para reutilização após o loop.
    */
    let names = vec!["Aula 1", "Aula 2", "Aula 3"];

    for name in names.iter() {
        match name {
            &"Aula 2" => println!("Encontrou a Atividade Aula 2!"),
            // TODO ^ Tente apagar o & para comparar apenas Aula 2
            _ => println!("Atividade: {}", name),
        }
    }

    println!("Atividades: {:?}", names);
}

pub fn controleFluxoForInteradores_into_iter() {
    /*
    into_iter - Consome a coleção para que a cada iteração os dados exatos sejam fornecidos.
    Depois que a coleção for consumida, ela não estará mais disponível para reutilização,
    pois foi “movida” dentro do loop.
    */

    let names = vec!["Aula 1", "Aula 2", "Aula 3"];

    for name in names.into_iter() {
        match name {
            "Aula2" => println!("Encontrou a Atividade Aula 2!"),
            _ => println!("Atividade {}", name),
        }
    }

    // println!("Atividade: {:?}", names);
    // FIXME ^ Comente esta linha
}


fn controleFluxoForInteradores_into_mut() {
    /*
    iter_mut - Empresta de forma mutável cada elemento da coleção,
    permitindo que a coleção seja modificada no local.
    */
    let mut names = vec!["Aula 1", "Aula 2", "Aula 3"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Aula 2" => "Encontrou a Atividade Aula 2!",
            _ => "Atividade",
        }
    }

    println!("Atividades: {:?}", names);
}