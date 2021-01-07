# rust-cookbook

[![gitlocalized ](https://gitlocalize.com/repo/3439/ru/badge.svg)](https://gitlocalize.com/repo/3439/ru?utm_source=badge)

Здесь ведётся перевод книги [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)

[Ссылка на проект в GitLocalize](https://gitlocalize.com/repo/3439/ru/rust-cookbook/)

## Замечания для переводчиков

### Локальная проверка правописания

Нужно сформировать Pull Request с переводом. GitLocalize поддерживает функцию формирования
и отправки существующего Review Request в качестве Pull Request. При этом создаётся специфичная
ветка вроде `gitlocalize-11866` и при вливании этого Pull Request соответствующий Review Request
автоматически закрывается.

Когда ветка уже существует, и нужно провести правки и проверку правописания локально, 
можно сделать следующее:
```bash
npm install yaspeller

git checkout gitlocalize-11866   # или что у вас там
git diff --name-only master --format="format:" \
    | grep --color=never -i '.md' \
    | xargs node_modules/.bin/yaspeller -c common-configs/.yaspellerr
```
