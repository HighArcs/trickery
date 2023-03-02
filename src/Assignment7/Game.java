import java.util.ArrayList;
import java.util.Scanner;

public class Game {
    public static void play(GameWheel g) {
        final Scanner s = new Scanner(System.in);
        Slice a = g.spinWheel();
        Slice b = g.spinWheel();
        Slice c = g.spinWheel();
        int sum = a.getPrizeAmount() + b.getPrizeAmount() + c.getPrizeAmount();

        System.out.println("Total prize money: $" + sum);
        System.out.println();
        System.out.println("Spin 1 - " + a.toString());
        System.out.println("Spin 2 - " + b.toString());
        System.out.println("Spin 3 - " + c.toString());
        
    }
}
