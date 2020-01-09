## Параллельный поиск элементов, соответствующих предикату

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

Этот пример использует [`rayon::find_any`](https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.find_any) и [`par_iter`](https://docs.rs/rayon/*/rayon/iter/trait.IntoParallelRefIterator.html#tymethod.par_iter) для параллельного поиска в векторе, удовлетворяющего предикату, который (предикат) передан как замыкание.

Если имеется несколько элементов, удовлетворяющих предикату, переданному в [`rayon::find_any`](https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.find_any), `rayon` возвращает первый попавшийся такой элемент, и это необязательно будет первый с начала вектора.

Также стоит заметить, что аргумент в замыкании это ссылка на ссылку (`&&x`). Стоит ознакомиться с документацией к [`std::find`], чтобы понять, почему сделано именно так.

```rust
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let v = vec![6, 2, 1, 9, 3, 8, 11];

    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    assert_eq!(f1, Some(&9));
    assert_eq!(f2, Some(&8));
    assert!(f3 > Some(&8));
}
```


[`std::find`]: https://docs.rs/rayon/*/rayon/iter/trait.IntoParallelRefIterator.html#tymethod.par_iter