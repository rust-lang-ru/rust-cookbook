## Сложение комплексных чисел

[![num-badge]][num] [![cat-science-badge]][cat-science]

Выполнение математических операций над комплексными числами является таким же как и для встроенных типов: рассматриваемые числа должны быть одного типа (то есть с плавающей точкой или целыми числами).

```rust
extern crate num;

fn main() {
    let complex_num1 = num::complex::Complex::new(10.0, 20.0); // Must use floats
    let complex_num2 = num::complex::Complex::new(3.1, -4.2);

    let sum = complex_num1 + complex_num2;

    println!("Sum: {}", sum);
}
```
