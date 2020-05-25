## Компиляция библиотеки на C с использованием особых директив

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

На самом деле довольно просто собрать код на C с особыми директивами используя [`cc::Build::define`]. Этот метод принимает значение [`Option`], таким образом делая возможным определить такие директивы, как, скажем, `#define APP_NAME "foo"` или там `#define WELCOME` (можно передать `None` для определения директивы без значения). Этот пример собирает файл на C с динамически определёнными директивами, установленными в `build.rs` и печатает "**Welcome to foo - version 1.0.2**" после запуска. Cargo устанавливает некоторые переменные окружения [environment variables], которые также могут быть полезны для динамического определения директив.

### `Cargo.toml`

```toml
[package]
...
version = "1.0.2"
build = "build.rs"

[build-dependencies]
cc = "1"
```

### `build.rs`

```rust,no_run
extern crate cc;

fn main() {
    cc::Build::new()
        .define("APP_NAME", "\"foo\"")
        .define("VERSION", format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str())
        .define("WELCOME", None)
        .file("src/foo.c")
        .compile("foo");
}
```

### `src/foo.c`

```c
#include <stdio.h>

void print_app_info() {
#ifdef WELCOME
    printf("Welcome to ");
#endif
    printf("%s - version %s\n", APP_NAME, VERSION);
}
```

### `src/main.rs`

```rust,ignore
extern {
    fn print_app_info();
}

fn main(){
    unsafe {
        print_app_info();
    }
}
```


[environment variables]: https://doc.rust-lang.org/cargo/reference/environment-variables.html