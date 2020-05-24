## Проверить количество логических ядер процессора

[![num_cpus-badge]][num_cpus] [![cat-hardware-support-badge]][cat-hardware-support]

Показывает количество логических ядер процессора на текущей машине, используя [ `num_cpus::get` ].

```rust
extern crate num_cpus;

fn main() {
    println!("Number of logical cores is {}", num_cpus::get());
}
```
