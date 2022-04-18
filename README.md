# Выводы по перформансу:
* Дарт: с ростом до 10 потоков производительность улучшается, тк задействуются системные потоки. 
Далее до 1000 изолятов вообще без разницы, если еще больше - начинает деградировать
* Джава и Раст: потоки стартуют медленно ок 5мс каждый поток, поэтому нужен пул.


### 1000 threads 100k rounds (100 each thread) on M1 2020
* Java: 8-12s (started in 5-6s)
* Rust: 38s (started in 5s)
* Dart: 7s (isolates started in 60-110ms)

### ON NEW M1 2021 as of March 2022
* JAVA: 2s (200ms start)  OpenJDK java 17
* RUST: 30s (4.3s start)
* Dart: 6.5s (100ms start)

Let's try to improve Rust version by minimizing heap allocations, 
see https://www.youtube.com/watch?v=nO-BFKyWYgE