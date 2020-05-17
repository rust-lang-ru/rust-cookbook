## Умножение скаляра на вектор и на матрицу

[![ndarray-badge]][ndarray] [![cat-science-badge]][cat-science]

Создаёт одномерный 1-D массив (вектор) с помощью [`ndarray::arr1`](https://docs.rs/ndarray/*/ndarray/fn.arr1.html) и двумерных 2-D массив (матрицу) с помощью [`ndarray::arr2`](https://docs.rs/ndarray/*/ndarray/fn.arr2.html).

Сначала скаляр умножается на вектор, чтобы получить другой вектор. Затем матрица умножается на новый вектор с помощью [`ndarray::Array2::dot`](https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#method.dot-1). (Матричное умножение выполняется с использованием метода `dot`, а оператор `*` выполняет поэлементное умножение.)

В `ndarray` массивы можно интерпретировать как либо векторы строк, либо столбцов, в зависимости от контекста. Если важно представить ориентацию вектора, необходимо использовать двумерный массив с одной строкой или одним столбцом. В этом примере вектор является 1-D массивом справа, поэтому `dot` обрабатывает его как вектор столбца.

```rust
extern crate ndarray;
use ndarray::{arr1, arr2, Array1};
fn main() {
    let scalar = 4;
    let vector = arr1(&[1, 2, 3]);
    let matrix = arr2(&[[4, 5, 6],
                        [7, 8, 9]]);
    let new_vector: Array1<_> = scalar * vector;
    println!("{}", new_vector);
    let new_matrix = matrix.dot(&new_vector);
    println!("{}", new_matrix);
}
```

