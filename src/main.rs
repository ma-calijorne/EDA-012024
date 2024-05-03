mod basics;
mod heap_stack;
mod tad;
mod vetoresArrays;
mod exerciciosSalaAula;
// mod hashMaps;
mod arvore_binaria;

fn main() {

    helloWorld();

    // basics::types::rs_types();
    // basics::controleFluxo::controleFluxoIfElse();
    // basics::stack_heap_usage::stackHeapUsage();
    // basics::controleFluxo::controleFluxoForInteradores_iter();
    // basics::controleFluxo::controleFluxoForInteradores_into_iter();
    // tad::tiposAbstratosDados::tadVector_3();
    // exerciciosSalaAula::Aula08032024::principal()
    // exerciciosSalaAula::exercicioCalculoAreaCirculo::calculoAreaCirculo()
    arvore_binaria::arvore_avl::main()
}

fn helloWorld(){

    println!("Olá Pessoal. Vamos mergulhar no mundo do RUST.");
}