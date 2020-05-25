## Замена всех вхождений одного текстового шаблона другим шаблоном.

[![regex-badge]][regex] [![lazy_static-badge]][lazy_static] [![cat-text-processing-badge]][cat-text-processing]

Заменяет все вхождения шаблона стандартной даты ISO 8601 *ГГГГ-ММ-ДД* на эквивалентную дату в американо-английский с косыми чертами. Например `2013-01-15` становится `01/15/2013` .

Метод [`Regex::replace_all`](https://docs.rs/regex/*/regex/struct.Regex.html#method.replace_all) заменяет все вхождения всего регулярного выражения. Тип `&str` реализует типаж `Replacer`, который позволяет переменным, таким как `$abcde` ссылаться на соответствующие именованные группы захвата `(?P<abcde>REGEX)` регулярного выражения поиска. Посмотрите [синтаксис строки замены](https://docs.rs/regex/*/regex/struct.Regex.html#replacement-string-syntax) для примеров и детали экранирования.

```rust
extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::borrow::Cow;
use regex::Regex;

fn reformat_dates(before: &str) -> Cow<str> {
    lazy_static! {
        static ref ISO8601_DATE_REGEX : Regex = Regex::new(
            r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})"
            ).unwrap();
    }
    ISO8601_DATE_REGEX.replace_all(before, "$m/$d/$y")
}

fn main() {
    let before = "2012-03-14, 2013-01-15 and 2014-07-05";
    let after = reformat_dates(before);
    assert_eq!(after, "03/14/2012, 01/15/2013 and 07/05/2014");
}
```


