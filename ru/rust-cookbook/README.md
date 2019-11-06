# Rust. Сборник рецептов &emsp; [![Build Status travis]][travis]

[Build Status travis]: https://api.travis-ci.com/rust-lang-nursery/rust-cookbook.svg?branch=master
[travis]: https://travis-ci.com/rust-lang-ru/rust-cookbook

**[Книга здесь]**

Этот *Сборник рецептов* является собранием маленьких примеров для языка программирования [Rust]. Используя крейты (crates) из экосистемы Rust эти примеры демонстрируют полезные идиомы и практики для выполнения обычных программистских задач

Эти примеры полностью готовы к использованию и подходят для простого копирования в какой-нибудь свежий проект на карго (cargo - стандартный пакетный менеджер для Rust). Они протестированы и гарантированно работают.

## Для чтения оффлайн

Если вам хочется скачать эту книгу локально:

```bash
$ git clone https://github.com/rust-lang-ru/rust-cookbook
$ cd rust-cookbook
$ cargo install mdbook --vers "0.1.8"
$ mdbook serve --open
```

Результат сборки также может быть открыт из каталога `book` в вашем браузере.

```bash
$ xdg-open ./book/index.html # linux
$ start .\book\index.html    # windows
$ open ./book/index.html     # mac
```

[Книга здесь]: https://rust-lang-ru.github.io/rust-cookbook
[Rust]: https://www.rust-lang.org/

## Помощь в разработке

Этот проект подразумевает лёгкость вхождения для новичков в программировании на
[Rust](https://www.rust-lang.org/), а также является одним из самых лёгких путей,
чтобы быть вовлечённым в Rust-сообщество. Помощь всегда приветствуется.

Подробнее об этом читайте [CONTRIBUTING.md] в репозитории на Github.

[CONTRIBUTING.md]: https://github.com/rust-lang-ru/rust-cookbook/blob/master/rust-cookbook/ru/CONTRIBUTING.md

## Лицензия [![CC0-badge]][CC0-deed]

_Rust. Сборник рецептов_  распространяется по лицензии Creative Commons Zero v1.0 Universal License.
([LICENSE-CC0](LICENSE-CC0) или https://creativecommons.org/publicdomain/zero/1.0/legalcode)

Если вы явно не заявляете об обратном, любой вклад преднамеренно представленный 
для включения в _Rust. Сборник рецептов_, как определено в лицензии CC0-1.0,
[становится общественным достоянием](CC0-deed) и лицензирован как указано выше, без каких-либо дополнительных
условий и положений.

[CC0-deed]: https://creativecommons.org/publicdomain/zero/1.0/deed.en
[CC0-badge]: https://mirrors.creativecommons.org/presskit/buttons/80x15/svg/cc-zero.svg
