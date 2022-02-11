
import 'package:test/test.dart';

import 'package:dart_bench/parser.dart';

void main() {
  test('count words', () {
    Result r = Parser(2).parse("We decided to go there. Go There, go there, go there. To be or not to be, that is the question.");

    expect(r.top_words, {'go': 4, 'there': 4});
    expect(r.topLetters, {'e': 15, 't': 12});
    print(r.top_words);
    print(r.topLetters);
  });
}
