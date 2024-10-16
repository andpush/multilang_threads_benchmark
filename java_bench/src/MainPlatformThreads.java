
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

import java.util.concurrent.*;

public class MainPlatformThreads {
    static final int THREADS = 1000;
    static final int ROUNDS = 100_000;
    static String text;

    public static void main(String[] args) throws IOException, InterruptedException, ExecutionException {
        text = Files.readString(Paths.get("text.txt"));
        long start = System.currentTimeMillis();
        Future<Parser.Result> f = null;
        try (ExecutorService es = Executors.newFixedThreadPool(THREADS)) {
            for (int i = 0; i < ROUNDS; i++) {
                f = es.submit(()-> new Parser(10).parse(text));
            }
            System.out.println("Started thread pool in " + (System.currentTimeMillis() - start) + " ms ");
            es.shutdown();
        }
        System.out.println("Finished processing in " + (System.currentTimeMillis() - start) + " ms ");
        System.out.println(f.get().topWords());
        System.out.println(f.get().topLetters());

    }


}


