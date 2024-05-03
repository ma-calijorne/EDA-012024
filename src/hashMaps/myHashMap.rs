/*
Neste exemplo, SimpleHashMap é uma implementação muito básica de uma tabela hash que utiliza
encadeamento para resolver colisões. Para simplificar, estamos usando o DefaultHasher do Rust, que
não é exatamente MurmurHash, mas serve para fins ilustrativos de como uma função hash poderia ser
aplicada. Esta implementação fornece funções básicas como insert, get, e remove.
*/
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Define o struct para armazenar as entradas da hashmap.
#[derive(Clone, Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

// Define o struct para a hashmap.
#[derive(Clone, Debug)]
struct SimpleHashMap<K, V> {
    buckets: Vec<Vec<Entry<K, V>>>, // Vetor de vetores para encadeamento
    capacity: usize,
}

impl<K, V> SimpleHashMap<K, V>
    where
        K: Hash + Eq, // K deve ser hashable e comparável
        V: Clone,     // V deve ser clonável
{
    // Cria uma nova SimpleHashMap com capacidade inicial.
    fn new(capacity: usize) -> Self {
        SimpleHashMap {
            buckets: vec![Vec::new(); capacity],
            capacity,
        }
    }

    // Calcula o índice do bucket para uma chave usando MurmurHash (simplificado).
    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    // Insere um novo par chave-valor na hashmap.
    fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        let bucket = &mut self.buckets[index];

        for entry in bucket.iter_mut() {
            if entry.key == key {
                entry.value = value;
                return;
            }
        }

        // Se a chave não existir, adiciona um novo Entry.
        bucket.push(Entry { key, value });
    }

    // Busca um valor na hashmap pela chave.
    fn get(&self, key: &K) -> Option<V> {
        let index = self.hash(key);
        self.buckets[index]
            .iter()
            .find(|entry| &entry.key == key)
            .map(|entry| entry.value.clone())
    }

    // Remove um par chave-valor pela chave.
    fn remove(&mut self, key: &K) {
        let index = self.hash(key);
        if let Some(position) = self.buckets[index].iter().position(|entry| &entry.key == key) {
            self.buckets[index].remove(position);
        }
    }
}

fn main() {
    let mut map = SimpleHashMap::new(10);
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    println!("{:?}", map.get(&"key1")); // Some("value1")
    println!("{:?}", map.get(&"key2")); // Some("value2")
    println!("{:?}", map.get(&"key3")); // None

    map.remove(&"key1");
    println!("{:?}", map.get(&"key1")); // None
}
