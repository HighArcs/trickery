public class tests {
    public static void main(String[] args) {
        Purchase x = new Purchase("gencorp");
        System.out.print(x);
    }
}

class Purchase {
    private int purchaseValue;
    private String customerName;
    private boolean paid;

    public Purchase(int p) {
        this(p, "customer");
    }

    public Purchase(String n) {
        this(100, n);
    }

    public Purchase(int p, String n) {
        purchaseValue = p;
        customerName = n;
        paid = false;
    }

    public String toString() {
        String s = "$" + purchaseValue + " from " + customerName + " - ";

        if (!paid) {
            s += "not ";
        }

        return s + "paid";
    }
}