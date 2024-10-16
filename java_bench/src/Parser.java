import java.util.*;


public class Parser {
    public int limit;

    public Parser(int limit) {
        this.limit = limit;
    }

    Result parse(String text) {
        Counter<String> wordCounter = new Counter<>();
        Counter<Character> letterCounter = new Counter<>();
        for (String word : text.toLowerCase().split("[\\s,.?!]+")) {
            wordCounter.add(word);
            for (Character letter : word.toCharArray()) {
                letterCounter.add(letter);
            }
        }
        return new Result(wordCounter.top(limit), letterCounter.top(limit));
    }


    public record Result (
        Map<String, Integer> topWords,
        Map<Character, Integer> topLetters
    ) {}


    static class Counter<T> {
        Map<T, Integer> _map = new HashMap<>();

        void add(T item) {
            int count = _map.getOrDefault(item, 0);
            _map.put(item, count + 1);
        }

        Map<T, Integer> top(int limit) {
            Map<T, Integer> result = new LinkedHashMap<>();
            _map.entrySet().stream()
                    .sorted((a, b) -> b.getValue().compareTo(a.getValue()))
                    .limit(limit)
                    .forEach(e -> result.put(e.getKey(), e.getValue()));
            return result;
        }
    }
}
