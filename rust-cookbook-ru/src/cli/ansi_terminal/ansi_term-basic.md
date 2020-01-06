## ANSI терминал

[![ansi_term-badge]][ansi_term] [![cat-command-line-badge]][cat-command-line]

Эта программа обрисовывает использование [`ansi_term` крейта] и как он используется для отображения текста с различным цветом, форматированием, к примеру, жирный синий или жёлтый подчёркнутый текст, на ANSI терминалах.

У нас в наличии две главных структуры данных в [`ansi_term`]: [`ANSIString`](https://docs.rs/ansi_term/*/ansi_term/type.ANSIString.html) и [`Style`](https://docs.rs/ansi_term/*/ansi_term/struct.Style.html). Структура [`Style`](https://docs.rs/ansi_term/*/ansi_term/struct.Style.html) содержит информацию о стиле: цвета, должен ли текст быть жирным или мигающим или ещё каким-то. Есть два варианта Colour, которые представляют простые цвета букв. Структура [`ANSIString`](https://docs.rs/ansi_term/*/ansi_term/type.ANSIString.html) -- это просто строка в паре со [`Style`](https://docs.rs/ansi_term/*/ansi_term/struct.Style.html).

**Примечание:** Если вы изучаете английский язык (а это скорее всего вещь обязательная для сферы IT), то стоит заметить, что в британском английском используется слово *Colour*, а в американском английском *Color*. При этом в библиотеке определён именно тип {em3}Colour{/em3}, не путайте.

### Вывод цветного текста на терминал

```rust
extern crate ansi_term;

use ansi_term::Colour;

fn main() {
    println!("This is {} in color, {} in color and {} in color",
             Colour::Red.paint("red"),
             Colour::Blue.paint("blue"),
             Colour::Green.paint("green"));
}
```

### Жирный текст в терминале

Для чего-то более сложного, чем простое изменения цвета переднего плана, нам уже нужно сконструировать экземпляр `Style`. Метод [`Style::new()`] создаёт экземпляр и её свойства устанавливаются цепочкой вызовов.

```rust
extern crate ansi_term;

use ansi_term::Style;

fn main() {
    println!("{} and this is not",
             Style::new().bold().paint("This is Bold"));
}
```

### Жирный и цветной текст в терминале

`Colour` реализует множество функций, наподобие функциям из `Style` и позволяет объединять вызовы в цепочку.

```rust
extern crate ansi_term;

use ansi_term::Colour;
use ansi_term::Style;

fn main(){
    println!("{}, {} and {}",
             Colour::Yellow.paint("This is colored"),
             Style::new().bold().paint("this is bold"),
             Colour::Yellow.bold().paint("this is bold and colored"));
}
```


[`ansi_term` крейта]: https://docs.rs/ansi_term/
[`Style::new()`]: https://crates.io/crates/ansi_term