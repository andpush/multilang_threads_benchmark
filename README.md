# Выводы по перформансу:
* Дарт: с ростом до 10 потоков производительность улучшается, тк задействуются системные потоки. 
Далее до 1000 изолятов вообще без разницы, если еще больше - начинает деградировать
* Джава и Раст: потоки стартуют медленно ок 5мс каждый поток, поэтому нужен пул.


### 1000 threads 100k rounds (100 each thread) on M1
* Java: 8-12s (started in 5-6s)
* Rust: 38s (started in 5s)
* Dart: 7s (isolates started in 60-110ms)