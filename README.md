# Выводы по перформансу:
* Дарт: с ростом до 10 потоков производительность улучшается, тк задействуются системные потоки. 
Далее до 1000 изолятов вообще без разницы, если еще больше - начинает деградировать
* Джава и Раст: потоки стартуют медленно ок 5мс каждый поток, поэтому нужен пул.

### 100 threads 10k rounds (100 each thread) on MacBook M1 Pro 2020:
* Java: 2200-2400ms (threads started in 350ms)
* Rust: 3900ms (threads started in 6 ms)
* Dart: 1000-1300ms (isolates started in 30ms)

### 1000 threads 100k rounds (100 each thread) on M1
* Java: 12s (started in 6s)
* Rust: 38s (started in 5s)
* Dart: 7s (isolates started in 130ms)