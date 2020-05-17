## Перемножение матриц

[![ndarray-badge]][ndarray] [![cat-science-badge]][cat-science]

Создаёт две матрицы с помощью [`ndarray::arr2`](https://docs.rs/ndarray/*/ndarray/fn.arr2.html) и выполняет перемножение матриц с помощью [`ndarray::ArrayBase::dot`](https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#method.dot-1).

```rust
extern crate ndarray;

use ndarray::arr2;

fn main() {
    let a = arr2(&[[1, 2, 3],
                   [4, 5, 6]]);
    let b = arr2(&[[6, 3],
                   [5, 2],
                   [4, 1]]);

    println!("{}", a.dot(&b));
}
```

