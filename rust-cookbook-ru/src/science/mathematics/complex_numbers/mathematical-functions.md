## Математические функции

[![num-badge]][num] [![cat-science-badge]][cat-science]

Комплексные числа обладают рядом интересных свойств, когда речь заходит о том как они взаимодействуют с другими математическими функциями, в частности с семейством синусоидальных функций, а также с числом e. Чтобы использовать эти функции с комплексными числами, у типа Complex есть несколько встроенных функций, все из которых можно найти здесь: [`num::complex::Complex`].

```rust
extern crate num;

use std::f64::consts::PI;
use num::complex::Complex;

fn main() {
    let x = Complex::new(0.0, 2.0*PI);

    println!("e^(2i * pi) = {}", x.exp()); // =~1
}
```


[`num::complex::Complex`]: https://autumnai.github.io/cuticula/num/complex/struct.Complex.html