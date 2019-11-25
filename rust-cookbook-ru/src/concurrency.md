# Конкурентность

Рецепт | Крейты | Категории
--- | --- | ---
[Запуск короткоживущего потока] | [![crossbeam-badge]][crossbeam] | [![cat-concurrency-badge]][cat-concurrency]
[Поддержание изменяемого глобального состояния] | [![lazy_static-badge]][lazy_static] | [![cat-rust-patterns-badge]][cat-rust-patterns]
[Параллельное вычисление SHA1 над множеством iso-файлов] | [![threadpool-badge]][threadpool] [![walkdir-badge]][walkdir] [![num_cpus-badge]][num_cpus] [![ring-badge]][ring] | [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem]
[Рисование фрактала с использованием пула потоков] | [![threadpool-badge]][threadpool] [![num-badge]][num] [![num_cpus-badge]][num_cpus] [![image-badge]][image] | [![cat-concurrency-badge]][cat-concurrency][![cat-science-badge]][cat-science][![cat-rendering-badge]][cat-rendering]
[Параллельное изменение элементов массива] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency]
[Параллельная проверка элементов коллекции на соответствие предикату] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency]
[Параллельный поиск элементов, соответствующих предикату] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency]
[Параллельная сортировка вектора] | [![rayon-badge]][rayon] [![rand-badge]][rand] | [![cat-concurrency-badge]][cat-concurrency]
[Параллельный map-reduce] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency]
[Параллельная генерация миниатюр для картинок в формате jpg] | [![rayon-badge]][rayon] [![glob-badge]][glob] [![image-badge]][image] | [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem]

{{#include links.md}}


[Запуск короткоживущего потока]: concurrency/threads.html#spawn-a-short-lived-thread
[Поддержание изменяемого глобального состояния]: concurrency/threads.html#maintain-global-mutable-state
[Параллельное вычисление SHA1 над множеством iso-файлов]: concurrency/threads.html#calculate-sha1-sum-of-iso-files-concurrently
[Рисование фрактала с использованием пула потоков]: concurrency/threads.html#draw-fractal-dispatching-work-to-a-thread-pool
[Параллельное изменение элементов массива]: concurrency/parallel.html#mutate-the-elements-of-an-array-in-parallel
[Параллельная проверка элементов коллекции на соответствие предикату]: concurrency/parallel.html#test-in-parallel-if-any-or-all-elements-of-a-collection-match-a-given-predicate
[Параллельный поиск элементов, соответствующих предикату]: concurrency/parallel.html#search-items-using-given-predicate-in-parallel
[Параллельная сортировка вектора]: concurrency/parallel.html#sort-a-vector-in-parallel
[Параллельный map-reduce]: concurrency/parallel.html#map-reduce-in-parallel
[Параллельная генерация миниатюр для картинок в формате jpg]: concurrency/parallel.html#generate-jpg-thumbnails-in-parallel