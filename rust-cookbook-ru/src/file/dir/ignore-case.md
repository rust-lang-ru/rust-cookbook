## Найти все файлы с заданным шаблоном, игнорируя регистр в имени файла

[![glob-badge]][glob] [![cat-filesystem-badge]][cat-filesystem]

Найдите все файлы изображений в каталоге `/media/` соответствующие образцу `img_[0-9]*.png`.

Пользовательская структура [`MatchOptions`](https://docs.rs/glob/*/glob/struct.MatchOptions.html) передаётся в функцию [`glob_with`](https://docs.rs/glob/*/glob/fn.glob_with.html) делая шаблон glob нечувствительным к регистру, оставляя другие параметры по умолчанию как [`Default`].

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate glob;

use glob::{glob_with, MatchOptions};
#
# error_chain! {
#     foreign_links {
#         Glob(glob::GlobError);
#         Pattern(glob::PatternError);
#     }
# }

fn main() -> Result<()> {
    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    for entry in glob_with("/media/img_[0-9]*.png", &options)? {
        println!("{}", entry?.display());
    }

    Ok(())
}
```


[`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html