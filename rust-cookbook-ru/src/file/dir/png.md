## Найти рекурсивно все png файлы

[![glob-badge]][glob] [![cat-filesystem-badge]][cat-filesystem]

Рекурсивно найти все файлы PNG в текущем каталоге. В данном случае шаблон `**` соответствует текущему каталогу и всем его подкаталогам.

Используйте шаблон `**` для любой части пути. Например, `/media/**/*.png` соответствует всем PNG в `media` и его подкаталогах.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate glob;

use glob::glob;
#
# error_chain! {
#     foreign_links {
#         Glob(glob::GlobError);
#         Pattern(glob::PatternError);
#     }
# }

fn main() -> Result<()> {
    for entry in glob("**/*.png")? {
        println!("{}", entry?.display());
    }

    Ok(())
}
```
