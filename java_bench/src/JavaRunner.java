
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

import java.util.concurrent.*;

public class JavaRunner {
    static final int THREADS = 100;
    static final int ROUNDS = 10000;
    static String text;

    public static void main(String[] args) throws IOException, InterruptedException, ExecutionException {
        text = Files.readString(Paths.get("text.txt"));
        long start = System.currentTimeMillis();
        ExecutorService es = Executors.newFixedThreadPool(THREADS);
        List<Future<Parser.Result>> ff = new ArrayList<>();
        for (int i=0; i<ROUNDS; i++) {
            ff.add(es.submit(JavaRunner::_doInBackground));
        }
        es.shutdown();
        System.out.println("Started thread pool in " + (System.currentTimeMillis() - start) + " ms ");

        if (!es.awaitTermination(1, TimeUnit.MINUTES)) {
            throw new RuntimeException();
        }

        System.out.println("Finished processing in " + (System.currentTimeMillis() - start) + " ms ");
        Future<Parser.Result> f = ff.get(0);
        System.out.println(f.get().topWords);
        System.out.println(f.get().topLetters);

        System.out.println("Finished all in " + (System.currentTimeMillis() - start) + " ms ");
    }

    private static Parser.Result _doInBackground() {
        return new Parser(10).parse(text);
    }


}


