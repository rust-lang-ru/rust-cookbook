# Rust. Сборник рецептов [![Build Status travis](https://api.travis-ci.com/rust-lang-nursery/rust-cookbook.svg?branch=master)  ](https://travis-ci.com/rust-lang-nursery/rust-cookbook)[![Build Status appveyor](https://ci.appveyor.com/api/projects/status/k56hklb7puv7c4he?svg=true)](https://ci.appveyor.com/project/rust-lang-libs/rust-cookbook)

**[Книга здесь]**.

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

## Помощь в разработке

Этот проект подразумевает лёгкость вхождения для новичков в программировании на [Rust](https://www.rust-lang.org/), а также является одним из самых лёгких путей, чтобы быть вовлечённым в Rust-сообщество. Помощь всегда приветствуется.

Подробнее об этом читайте [CONTRIBUTING.md] в репозитории на Github.

## Лицензия [![CC0-badge]](https://creativecommons.org/publicdomain/zero/1.0/deed.en)

{em0}Rust. Сборник рецептов{/em0}  распространяется по лицензии Creative Commons Zero v1.0 Universal License.
( [LICENSE-CC0](LICENSE-CC0) или https://creativecommons.org/publicdomain/zero/1.0/legalcode)

Если вы явно не заявляете об обратном, любой вклад преднамеренно представленный для включения в _Rust. Сборник рецептов_, как определено в лицензии CC0-1.0,
[становится общественным достоянием] и лицензирован, как указано выше, без каких-либо дополнительных
условий и положений.


[Книга здесь]: https://api.travis-ci.com/rust-lang-nursery/rust-cookbook.svg?branch=master
[Rust]: https://travis-ci.com/rust-lang-nursery/rust-cookbook
[CONTRIBUTING.md]: https://ci.appveyor.com/api/projects/status/k56hklb7puv7c4he?svg=true
[становится общественным достоянием]: https://ci.appveyor.com/project/rust-lang-libs/rust-cookbook
[CC0-badge]: https://rust-lang-nursery.github.io/rust-cookbook