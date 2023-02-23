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

#### UPD Dec 2022: Same computer (M1), after update to MacOS Ventura, Rust 1.66, Dart 2.18:
* JAVA: same
* RUST: 21s (4s start)
* Dart: 4s (30ms start)

Let's try to improve Rust version by minimizing heap allocations, 
see https://www.youtube.com/watch?v=nO-BFKyWYgE

#### UPD Feb 2023 (updated Rust to 1.67.1 some code changes + Tokio Async version)
* Rust  Optimized 110 ms start, total 2.6s - @ Intel
* Rust Tokio Optimized 80 ms start, total 2.3s - @ Intel
* Java (OpenJDK 17.0.2) 12s (2.6s for start) - @ Intel
* Dart 7s - @ Intel
* All versions load the CPU equally to 1200% 