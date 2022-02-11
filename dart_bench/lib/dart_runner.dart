import 'package:dart_bench/parser.dart';
import 'dart:io';
import 'dart:isolate';
import 'dart:async';

const threads = 1000, rounds = 1000000;
final String text = File("../text.txt").readAsStringSync();
main() async {
  final sw = Stopwatch()
    ..start();

  var collector = <Future<Result>>[];
  for (int i = 0; i < threads; i++) {
    collector.add(spawnThreads());
  }

  print("Isolates created in ${sw.elapsedMilliseconds} ms");

  await Future.wait(collector);

  print("Isolates finished in ${sw.elapsedMilliseconds} ms");

  Result r = await collector[0];
  print("${r.topWords}\n${r.topLetters}");

}

// Spawns an isolate
Future<Result> spawnThreads() async {
  final p = ReceivePort();
  await Isolate.spawn(_doWork, p.sendPort);
  return await p.first;
}

void _doWork(SendPort sendPort) {
  late Result result;
  for (int i = 0; i < rounds / threads; i++) {
    result = Parser(10).parse(text);
  }
  sendPort.send(result);
}

