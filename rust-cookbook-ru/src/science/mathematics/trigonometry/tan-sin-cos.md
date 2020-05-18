## Проверка того, что тангенс равен синусу, разделённому на косинус

[![std-badge]][std] [![cat-science-badge]][cat-science]

Проверяет, что tan (x) равен sin (x) / cos (x) для x = 6.

```rust
fn main() {
    let x: f64 = 6.0;

    let a = x.tan();
    let b = x.sin() / x.cos();

    assert_eq!(a, b);
}
```
