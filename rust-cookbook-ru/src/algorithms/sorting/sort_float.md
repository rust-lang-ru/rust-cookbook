## Сортировка вектора вещественных чисел

[![std-badge]][std] [![cat-science-badge]][cat-science]

Вектор из f32 or f64 может быть отсортирован с помощью методов [`vec::sort_by`] и [`PartialOrd::partial_cmp`].

```rust
fn main() {
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}
```


[`vec::sort_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by
[`PartialOrd::partial_cmp`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#tymethod.partial_cmp