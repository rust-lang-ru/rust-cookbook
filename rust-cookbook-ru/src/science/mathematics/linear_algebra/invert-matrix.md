## Инвертирование матрицы

[![nalgebra-badge]][nalgebra] [![cat-science-badge]][cat-science]

Создаёт матрицу 3x3 с помощью [`nalgebra::Matrix3`](https://docs.rs/nalgebra/*/nalgebra/base/type.Matrix3.html) и инвертирует её, если возможно.

```rust
extern crate nalgebra;

use nalgebra::Matrix3;

fn main() {
    let m1 = Matrix3::new(2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0);
    println!("m1 = {}", m1);
    match m1.try_inverse() {
        Some(inv) => {
            println!("The inverse of m1 is: {}", inv);
        }
        None => {
            println!("m1 is not invertible!");
        }
    }
}
```

