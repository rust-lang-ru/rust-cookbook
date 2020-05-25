## Компиляция и статическая компоновка с идущей в комплекте библиотекой на C++

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

Компоновка с библиотекой на C++ очень похожа на компоновку с библиотекой на C. Самые главные два отличия это когда вам нужно компилировать и компоновать библиотеку на C++ вы должны определить компилятор C++ с помощью специального метода [`cpp(true)`](https://docs.rs/cc/*/cc/struct.Build.html#method.cpp) и также нужно предотвратить искажение имён компилятором C++ путём добавления `extern "C"` секции на самый верхний уровень в исходном коде C++.

### `Cargo.toml`

```toml
[package]
...
build = "build.rs"

[build-dependencies]
cc = "1"
```

### `build.rs`

```rust,no_run
extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/foo.cpp")
        .compile("foo");
}
```

### `src/foo.cpp`

```cpp
extern "C" {
    int multiply(int x, int y);
}

int multiply(int x, int y) {
    return x*y;
}
```

### `src/main.rs`

```rust,ignore
extern {
    fn multiply(x : i32, y : i32) -> i32;
}

fn main(){
    unsafe {
        println!("{}", multiply(5,7));
    }
}
```


