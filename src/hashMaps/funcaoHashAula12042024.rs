/*
Explicação da Função Hash
* key é o número da reserva.
* hash_size é o número de baldes no hashmap.
* A constante A é a proporção áurea menos um, uma escolha popular para minimizar a clumping em
funções hash de multiplicação.
* A função math.modf() é usada para separar a parte fracionária da multiplicação, que é
então escalada pelo tamanho do hash para obter o índice.
Este tipo de função hash é eficaz porque é rápido e distribui as chaves uniformemente,
especialmente se o tamanho do hash for escolhido para ser um número que não é muito próximo
de uma potência de 2. Esta abordagem minimiza as colisões e é fácil de implementar. É uma
alternativa eficaz quando não se deseja utilizar funções hash mais complexas e patenteadas
ou criptográficas.
*/

fn hash_function(key: u32, hash_size: usize) -> usize {
    let a: f64 = 0.6180339887;  // A constante irracional, aproximadamente (sqrt(5) - 1) / 2
    let temp: f64 = (key as f64) * a;  // Multiplica a chave pelo número irracional
    let frac: f64 = temp.fract();  // Método .fract() para obter a parte fracionária de um número em Rust
    let hash_index: usize = (frac * (hash_size as f64)) as usize;  // Converte o resultado fracionário escalado para um índice inteiro
    hash_index
}