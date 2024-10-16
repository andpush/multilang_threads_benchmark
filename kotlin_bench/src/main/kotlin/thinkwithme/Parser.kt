package thinkwithme

import java.util.*

class Parser(var limit: Int) {
    fun parse(text: String): Result {
        val wordCounter = Counter<String>()
        val letterCounter = Counter<Char>()
        for (word in text.lowercase(Locale.getDefault()).split("[\\s,.?!]+".toRegex()).dropLastWhile { it.isEmpty() }
            .toTypedArray()) {
            wordCounter.add(word)
            for (letter in word.toCharArray()) {
                letterCounter.add(letter)
            }
        }
        return Result(wordCounter.top(limit), letterCounter.top(limit))
    }


    data class Result(val topWords: Map<String, Int>, val topLetters: Map<Char, Int>)


    internal class Counter<T> {
        private val _map: MutableMap<T, Int> = HashMap()

        fun add(item: T) {
            val count = _map.getOrDefault(item, 0)
            _map[item] = count + 1
        }

        fun top(limit: Int): Map<T, Int> {
            val result: MutableMap<T, Int> = LinkedHashMap()
            _map.entries.stream()
                .sorted { a: Map.Entry<T, Int>, b: Map.Entry<T, Int> -> b.value.compareTo(a.value) }
                .limit(limit.toLong())
                .forEach { e: Map.Entry<T, Int> -> result[e.key] = e.value }
            return result
        }
    }
}