library dart_test;

import 'dart:collection';
import 'package:characters/characters.dart';

class Parser {
  final int limit;

  const Parser(this.limit);

  Result parse(String text) {
    final _wordCounter = Counter();
    final _letterCounter = Counter();
    text.toLowerCase().split(RegExp("[\\s,.?!]+")).forEach((word) {
        _wordCounter.add(word);
        for (var ch in word.characters) {_letterCounter.add(ch);}
    });
    return Result(_wordCounter.top(limit), _letterCounter.top(limit));
  }
}

class Counter {
  final _map = <String, int>{};

  void add(String item) {
    final count = _map[item];
    _map[item] = (count ?? 0) + 1;
  }

  Map<String, int> top(int limit) {
    return LinkedHashMap.fromEntries(
        (List<MapEntry<String, int>>.from(_map.entries, growable: false)
          ..sort((a, b) => b.value.compareTo(a.value))
        ).take(limit)
    );
  }
}

class Result {
  final Map<String, int> top_words;
  final Map<String, int> topLetters;
  const Result(this.top_words, this.topLetters);
}

