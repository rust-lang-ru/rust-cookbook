## Сортировка вектора целых чисел

[![std-badge]][std] [![cat-science-badge]][cat-science]

Этот пример сортирует вектор целых чисел с помощью метода [`vec::sort`]. Альтернативой было бы использовать  [`vec::sort_unstable`], который может быть быстрее в некоторых случаях, но не гарантирует сохранение порядка равных элементов.

```rust
fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];
    
    vec.sort();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}
```


[`vec::sort`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort
[`vec::sort_unstable`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_unstable