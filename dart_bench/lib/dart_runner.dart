import 'package:dart_bench/parser.dart';
import 'dart:io';
import 'dart:isolate';

const THREADS = 1000, ROUNDS = 100000;
final String text = File("../text.txt").readAsStringSync();
main() async {
  final sw = Stopwatch()
    ..start();
  final p = ReceivePort();
  for (int i = 0; i < THREADS; i++) {
    Isolate.spawn(_bgTask, p.sendPort);
  }
  print("Isolates created in ${sw.elapsedMilliseconds} ms");
  // wait till all results received
  Result r = await p.take(ROUNDS).last;
  print("Isolates finished in ${sw.elapsedMilliseconds} ms");
  print("${r.topWords}\n${r.topLetters}");
}


void _bgTask(SendPort sendPort) {
  for (int i = 0; i < ROUNDS / THREADS; i++) {
    sendPort.send(Parser(10).parse(text));
  }
}

