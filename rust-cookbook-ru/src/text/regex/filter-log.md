## Фильтрация лог файла сопоставлением нескольких регулярных выражений

[![regex-badge]][regex] [![cat-text-processing-badge]][cat-text-processing]

Чтение файла с именем `application.log` и вывод только строк, содержащих «версию XXX», некоторый IP-адрес за которым следует порт 443 (например, «192.168.0.1:443») или специальное предупреждение.

Структура [`regex::RegexSetBuilder`](https://docs.rs/regex/*/regex/struct.RegexSetBuilder.html) составляет [`regex::RegexSet`](https://docs.rs/regex/*/regex/struct.RegexSet.html). Поскольку обратные слеши очень распространены в регулярных выражениях, использование [необработанных строковых литералов](https://doc.rust-lang.org/reference/tokens.html#raw-string-literals) делает их более читабельными.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::RegexSetBuilder;

# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         Regex(regex::Error);
#     }
# }
#
fn main() -> Result<()> {
    let log_path = "application.log";
    let buffered = BufReader::new(File::open(log_path)?);

    let set = RegexSetBuilder::new(&[
        r#"version "\d\.\d\.\d""#,
        r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:443"#,
        r#"warning.*timeout expired"#,
    ]).case_insensitive(true)
        .build()?;

    buffered
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| set.is_match(line.as_str()))
        .for_each(|x| println!("{}", x));

    Ok(())
}
```


