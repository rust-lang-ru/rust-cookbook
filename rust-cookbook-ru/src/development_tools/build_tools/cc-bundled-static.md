## Компиляция и статическая компоновка с идущей в комплекте библиотекой на C

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

Чтобы покрыть сценарии, где требуется использовать дополнительный код на C, C++ или ассемблере, есть крейт [**cc**][cc], который предлагает простой API для компиляции включённого в проект кода на C/C++/asm в статические библиотеки (**.a**), которые затем могут быть статически скомпонованы **rustc**.

Следующий пример содержит некоторый код на C (**src/hello.c**) который будет использован в Rust коде. Перед компиляцией кода на Rust файл запускается специальный "build" файл (**build.rs**), определённый в **Cargo.toml**. Используя крейт [**cc**][cc] будет создана статическая библиотека (в данном случае **libhello.a**, смотрите [`compile` docs](https://docs.rs/cc/*/cc/struct.Build.html#method.compile)), которая затем может быть использована из кода на Rust с помощью декларации сигнатуры внешней функции в блоке `extern`.

Поскольку включённый код на C очень простой, нужен только один файл с исходным кодом нужен для передачи в [`cc::Build`](https://docs.rs/cc/*/cc/struct.Build.html). В случае более сложных сценариев, [`cc::Build`](https://docs.rs/cc/*/cc/struct.Build.html) предлагает полный набор возможностей для определения [`include`](https://docs.rs/cc/*/cc/struct.Build.html#method.include) путей и флагов [`flag`](https://docs.rs/cc/*/cc/struct.Build.html#method.flag) для внешнего компилятора.

### `Cargo.toml`

```toml
[package]
...
build = "build.rs"

[build-dependencies]
cc = "1"

[dependencies]
error-chain = "0.11"
```

### `build.rs`

```rust,no_run
extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");   // outputs `libhello.a`
}
```

### `src/hello.c`

```c
#include <stdio.h>


void hello() {
    printf("Hello from C!\n");
}

void greet(const char* name) {
    printf("Hello, %s!\n", name);
}
```

### `src/main.rs`

```rust,ignore
# #[macro_use] extern crate error_chain;
use std::ffi::CString;
use std::os::raw::c_char;
#
# error_chain! {
#     foreign_links {
#         NulError(::std::ffi::NulError);
#         Io(::std::io::Error);
#     }
# }
#
# fn prompt(s: &str) -> Result<String> {
#     use std::io::Write;
#     print!("{}", s);
#     std::io::stdout().flush()?;
#     let mut input = String::new();
#     std::io::stdin().read_line(&mut input)?;
#     Ok(input.trim().to_string())
# }

extern {
    fn hello();
    fn greet(name: *const c_char);
}

fn main() -> Result<()> {
    unsafe { hello() }
    let name = prompt("What's your name? ")?;
    let c_name = CString::new(name)?;
    unsafe { greet(c_name.as_ptr()) }
    Ok(())
}
```


