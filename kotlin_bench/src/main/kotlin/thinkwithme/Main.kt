package thinkwithme

import kotlinx.coroutines.*
import java.nio.file.Files
import java.nio.file.Paths
import kotlin.system.measureTimeMillis

val THREADS = 1000
val ROUNDS = 100000
lateinit var text: String;

fun main() = runBlocking {
    text = Files.readString(Paths.get("text.txt"))
    var result: Parser.Result? = null
    val time = measureTimeMillis {
        // launch coroutines to run in parallel many background tasks
        val jobs = List(THREADS) {
            launch (Dispatchers.Default) {
                for (i in 1..ROUNDS/THREADS) {
                    result = bgTask()
                }
            }
        }
        jobs.forEach { it.join() }
    }
    println("Finished in in $time ms \n $result")
}

fun bgTask():Parser.Result {
    return Parser(10).parse(text);
}