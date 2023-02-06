import java.lang.management.MemoryUsage;
import java.util.ArrayList;

public class GameWheel {
    private ArrayList<Slice> slices; // List of slices making up the wheel
    private int currentPos; // Position of currently selected slice on wheel

    /*
     * Returns string representation of GameWheel with each numbered slice
     * on a new line
     */
    public String toString() {
        // Implement the toString method here
        String out = "";
        for (int index = 0; index < this.slices.size(); index++) {
            out += index;
            out += " - ";
            out += this.slices.get(index).toString();
            out += "\n";
        }

        return out.trim(); // remove trailing \n
    }

    /*
     * Randomizes the positions of the slices that are in the wheel, but without
     * changing the pattern of the colors
     */
    public void scramble() {
        ArrayList<Slice> black = new ArrayList<>();
        ArrayList<Slice> blue = new ArrayList<>();
        ArrayList<Slice> red = new ArrayList<>();

        for (Slice slice : this.slices) {
            if (slice.getColor().equals("black")) {
                black.add(slice);
            } else if (slice.getColor().equals("blue")) {
                blue.add(slice);
            } else if (slice.getColor().equals("red")) {
                red.add(slice);
            } else {
                throw new Error("invalid slice");
            }
        }

        ArrayList<Slice> out = new ArrayList<>();

        for (int index = 0; index < slices.size(); index++) {
            if (index % 5 == 0) {
                // pick!
                out.add(black.remove(random_idx(black)));
            } else if (index % 2 == 0) {
                out.add(blue.remove(random_idx(blue)));
            } else {
                out.add(red.remove(random_idx(red)));
            }
        }

        this.slices = out;
    }

    private <T> int random_idx(ArrayList<T> list) {
        // optimizations here
        if (list.size() == 0) {
            throw new Error("cannot get indexes of empty lists");
        }

        if (list.size() == 1) {
            return 0;
        }

        return (int) (Math.random() * ((double) list.size()));
    }

    /*
     * Sorts the positions of the slices that are in the wheel by prize amount,
     * but without changing the pattern of the colors.
     */
    public void sort() {
        for (int j = 1; j < this.slices.size(); j++) {
            Slice current = this.slices.get(j);
            int i = j - 1;
            while ((i > -1) && (this.slices.get(i).getPrizeAmount() > current.getPrizeAmount())) {
                this.slices.set(i + 1, this.slices.get(i));
                i--;
            }

            this.slices.set(i + 1, this.slices.get(i));
        }
    }

    /* COMPLETED METHODS - YOU DO NOT NEED TO CHANGE THESE */

    /*
     * Creates a wheel with 20 preset slices
     */
    public GameWheel() {
        this(getStandardPrizes());
    }

    /*
     * Creates a wheel with 20 slices, using values from array parameter
     */
    public GameWheel(int[] prizes) {
        currentPos = 0;
        slices = new ArrayList<Slice>();
        for (int i = 0; i < 20; i++) {
            int pa = 0;
            String col = "blue";

            if (i < prizes.length) {
                pa = prizes[i];
            }

            if (i % 5 == 0) {
                col = "black";
            } else if (i % 2 == 1) {
                col = "red";
            }

            slices.add(new Slice(col, pa));
        }
    }

    /*
     * Spins the wheel by so that a different slice is selected. Returns that
     * slice (Note: the 10 slices following the current slice are more likely to
     * be returned than the other 10).
     */
    public Slice spinWheel() {
        // spin power between range of 1-50 (inclusive)
        int power = (int) (Math.random() * 50 + 1);
        int newPos = (currentPos + power) % slices.size();
        currentPos = newPos;
        return slices.get(currentPos);
    }

    public Slice getSlice(int i) {
        int sliceNum = i;
        if (i < 0 || i > 19)
            sliceNum = 0;
        return slices.get(sliceNum);
    }

    // Makes an array with a standard list of prizes
    private static int[] getStandardPrizes() {
        int[] arr = new int[20];
        for (int i = 0; i < 20; i++) {
            if (i % 5 == 0)
                arr[i] = i * 1000;
            else if (i % 2 == 1)
                arr[i] = i * 100;
            else
                arr[i] = i * 200;
        }
        return arr;
    }
}
