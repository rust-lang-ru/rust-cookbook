# Веб-программирование

## Обкачка веб сайтов

Рецепт | Крейты | Категории
--- | --- | ---
[Извлечение всех ссылок из HTML-страницы] | [![reqwest-badge]][reqwest] [![select-badge]][select] | [![cat-net-badge]][cat-net]
[Проверка вебстраницы на наличие мёртвых ссылок] | [![reqwest-badge]][reqwest] [![select-badge]][select] [![url-badge]][url] | [![cat-net-badge]][cat-net]
[Извлечение всех уникальных ссылок из страницы в формате MediaWiki] | [![reqwest-badge]][reqwest] [![regex-badge]][regex] | [![cat-net-badge]][cat-net]

## Работа с унифицированными указателями ресурсов (URL)

Рецепт | Крейты | Категории
--- | --- | ---
[Парсинг URL из строки в значение типа `Url`] | [![url-badge]][url] | [![cat-net-badge]][cat-net]
[Создание базового URL удалением сегментов пути] | [![url-badge]][url] | [![cat-net-badge]][cat-net]
[Создание новых URL из базового URL] | [![url-badge]][url] | [![cat-net-badge]][cat-net]
[Извлечение URL источника (тройки схема / хост / порт)] | [![url-badge]][url] | [![cat-net-badge]][cat-net]
[Удаление параметров запроса из URL] | [![url-badge]][url] | [![cat-net-badge]][cat-net]

## Типы мультимедиа (MIME)

Рецепт | Крейты | Категории
--- | --- | ---
[Получение типа MIME из строки] | [![mime-badge]][mime] | [![cat-encoding-badge]][cat-encoding]
[Получение типа MIME из имени файла] | [![mime-badge]][mime] | [![cat-encoding-badge]][cat-encoding]
[Парсинг типа MIME из HTTP ответа] | [![mime-badge]][mime] [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

{{#include web/clients.md}}

{{#include links.md}}


[Извлечение всех ссылок из HTML-страницы]: web/scraping.html#extract-all-links-from-a-webpage-html
[Проверка вебстраницы на наличие мёртвых ссылок]: web/scraping.html#check-a-webpage-for-broken-links
[Извлечение всех уникальных ссылок из страницы в формате MediaWiki]: web/scraping.html#extract-all-unique-links-from-a-mediawiki-markup
[Парсинг URL из строки в значение типа `Url`]: web/url.html#parse-a-url-from-a-string-to-a-url-type
[Создание базового URL удалением сегментов пути]: web/url.html#create-a-base-url-by-removing-path-segments
[Создание новых URL из базового URL]: web/url.html#create-new-urls-from-a-base-url
[Извлечение URL источника (тройки схема / хост / порт)]: web/url.html#extract-the-url-origin-scheme--host--port
[Удаление параметров запроса из URL]: web/url.html#remove-fragment-identifiers-and-query-pairs-from-a-url
[Получение типа MIME из строки]: web/mime.html#get-mime-type-from-string
[Получение типа MIME из имени файла]: web/mime.html#get-mime-type-from-filename
[Парсинг типа MIME из HTTP ответа]: web/mime.html#parse-the-mime-type-of-a-http-response