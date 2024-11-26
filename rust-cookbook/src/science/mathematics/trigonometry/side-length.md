## Calculating the side length of a triangle

[![std-badge]][std] [![cat-science-badge]][cat-science]

Calculates the length of the hypotenuse of a right-angle triangle with an angle of 1 radian and opposite side length of 80.

```rust,edition2018
fn main() {
    let angle: f64 = 1.0;
    let side_length = 80.0;

    let hypotenuse = side_length / angle.sin();

    println!("Hypotenuse: {}", hypotenuse);
}
```
