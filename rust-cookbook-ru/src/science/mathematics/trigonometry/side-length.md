## Расчёт длины стороны треугольника

[![std-badge]][std] [![cat-science-badge]][cat-science]

Вычисляет длину гипотенузы прямоугольного треугольника с углом 2 радиана и противоположной стороной длиной 80.

```rust
fn main() {
    let angle: f64 = 2.0;
    let side_length = 80.0;

    let hypotenuse = side_length / angle.sin();

    println!("Hypotenuse: {}", hypotenuse);
}
```
