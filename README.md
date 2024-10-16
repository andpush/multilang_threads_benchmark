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

#### UPD Feb 2023 (some minor changes + Tokio Async version) start/total
* Rust  Optimized 110ms/2.6s @ Intel/rust1.67.1; 65ms/1.5s @ M1/rust1.67.1
* Rust Tokio Optimized 80m/2.3s @ Intel/rust1.67.1; 19ms/1.4s @ M1/rust1.67.1
* Java 2.6s/12s @ Intel/openjdk17; 250ms/1.6s @ M1/openjdk19 
* Dart 30ms/7s @ Intel; 25ms/4s @ M1/dart2.19
* All versions fully load the CPU

#### UPD Oct 2024 (added Kotlin 2.0.10, JRE Temurin 21.0.4) Threads=1000, Rounds=100_000, Macbook Pro 2023 M3 Pro
* Java 130/3400ms (if 100 threads - 180/2300ms) 
* Kotlin 900ms 
* Dart 10ms/1680ms 