## Сложение матриц

[![ndarray-badge]][ndarray] [![cat-science-badge]][cat-science]

Создает две двумерные матрицы (2-D) с помощью [`ndarray::arr2`](https://docs.rs/ndarray/*/ndarray/fn.arr2.html) и суммирует их поэлементно.

Note the sum is computed as `let sum = &a + &b`. The `&` operator is used to avoid consuming `a` and `b`, making them available later for display. A new array is created containing their sum.

```rust
extern crate ndarray;

use ndarray::arr2;

fn main() {
    let a = arr2(&[[1, 2, 3],
                   [4, 5, 6]]);

    let b = arr2(&[[6, 5, 4],
                   [3, 2, 1]]);

    let sum = &a + &b;

    println!("{}", a);
    println!("+");
    println!("{}", b);
    println!("=");
    println!("{}", sum);
}
```
