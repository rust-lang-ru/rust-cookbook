## Генерация случайных чисел

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Пример генерирует случайные числа с помощью генератора (псевдо)случайных чисел [`rand::Rng`](https://docs.rs/rand/*/rand/trait.Rng.html) полученного через [`rand::thread_rng`](https://docs.rs/rand/*/rand/fn.thread_rng.html). Каждый поток операционной системы имеет свой инициализированный генератор. Сгенерированные целые числа распределены равномерно на диапазоне, заданного их типом, а вещественные числа генерируются из диапазона от 0 до 1, не включая само число 1.

```rust
extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}
```


