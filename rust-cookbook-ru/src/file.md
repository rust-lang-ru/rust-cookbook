# Файловая система

Рецепт | Крейты | Категории
--- | --- | ---
[Чтение файла построчно] | [![std-badge]][std] | [![cat-filesystem-badge]][cat-filesystem]
[Избегание записи  и чтения из одного и того же файла] | [![same_file-badge]][same_file] | [![cat-filesystem-badge]][cat-filesystem]
[Произвольный доступ к файлу с использованием проекции в память (memory map)] | [![memmap-badge]][memmap] | [![cat-filesystem-badge]][cat-filesystem]
[Получение файлов, которые были модифицированы за последние 24 часа] | [![std-badge]][std] | [![cat-filesystem-badge]][cat-filesystem] [![cat-os-badge]][cat-os]
[Нахождение циклов в заданном пути] | [![same_file-badge]][same_file] | [![cat-filesystem-badge]][cat-filesystem]
[Рекурсивное нахождение одинаковых имён файлов] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem]
[Рекурсивное нахождение всех файлов, удовлетворяющим заданному предикату] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem]
[Обход каталога с пропуском файлов, начинающихся с точки (dotfiles)] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem]
[Рекурсивное вычисление размеров файлов по заданному пути] | [![walkdir-badge]][walkdir] | [![cat-filesystem-badge]][cat-filesystem]
[Рекурсивное нахождение всех png-файлов] | [![glob-badge]][glob] | [![cat-filesystem-badge]][cat-filesystem]
[Нахождение всех файлов по заданному паттерну без учёта регистра символов] | [![glob-badge]][glob] | [![cat-filesystem-badge]][cat-filesystem]

{{#include links.md}}


[Чтение файла построчно]: file/read-write.html#read-lines-of-strings-from-a-file
[Избегание записи  и чтения из одного и того же файла]: file/read-write.html#avoid-writing-and-reading-from-a-same-file
[Произвольный доступ к файлу с использованием проекции в память (memory map)]: file/read-write.html#access-a-file-randomly-using-a-memory-map
[Получение файлов, которые были модифицированы за последние 24 часа]: file/dir.html#file-names-that-have-been-modified-in-the-last-24-hours
[Нахождение циклов в заданном пути]: file/dir.html#find-loops-for-a-given-path
[Рекурсивное нахождение одинаковых имён файлов]: file/dir.html#recursively-find-duplicate-file-names
[Рекурсивное нахождение всех файлов, удовлетворяющим заданному предикату]: file/dir.html#recursively-find-all-files-with-given-predicate
[Обход каталога с пропуском файлов, начинающихся с точки (dotfiles)]: file/dir.html#traverse-directories-while-skipping-dotfiles
[Рекурсивное вычисление размеров файлов по заданному пути]: file/dir.html#recursively-calculate-file-sizes-at-given-depth
[Рекурсивное нахождение всех png-файлов]: file/dir.html#find-all-png-files-recursively
[Нахождение всех файлов по заданному паттерну без учёта регистра символов]: file/dir.html#find-all-files-with-given-pattern-ignoring-filename-case