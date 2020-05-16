## Создание комплексных чисел

[![num-badge]][num] [![cat-science-badge]][cat-science]

Создает комплексные числа типа [`num::complex::Complex`]. И действительная, и мнимая части комплексного числа должны быть одного типа.

```rust
extern crate num;

fn main() {
    let complex_integer = num::complex::Complex::new(10, 20);
    let complex_float = num::complex::Complex::new(10.1, 20.1);

    println!("Complex integer: {}", complex_integer);
    println!("Complex float: {}", complex_float);
}
```


[`num::complex::Complex`]: https://autumnai.github.io/cuticula/num/complex/struct.Complex.html