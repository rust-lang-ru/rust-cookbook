## Сбор Unicode графем

[![unicode-segmentation-badge]](https://docs.rs/unicode-segmentation/1.2.1/unicode_segmentation/) [![cat-text-processing-badge]][cat-text-processing]

Собрать отдельные Unicode графемы из строки UTF-8, используя функцию [`UnicodeSegmentation::graphemes`](https://docs.rs/unicode-segmentation/*/unicode_segmentation/trait.UnicodeSegmentation.html#tymethod.graphemes) из крейта [`unicode-segmentation`].

```rust
#[macro_use]
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let name = "José Guimarães\r\n";
    let graphemes = UnicodeSegmentation::graphemes(name, true)
    	.collect::<Vec<&str>>();
	assert_eq!(graphemes[3], "é");
}
```


[`unicode-segmentation`]: https://docs.rs/unicode-segmentation/*/unicode_segmentation/trait.UnicodeSegmentation.html#tymethod.graphemes