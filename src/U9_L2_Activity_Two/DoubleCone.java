public class DoubleCone extends Cone {
    private String flavor1;
    private String flavor2;
    private String topping;

    public DoubleCone(String f, boolean w) {
        super(f, w);
        this.flavor1 = f;
        this.flavor2 = f;
    }

    public DoubleCone(String f1, String f2, boolean w) {
        this(f1, w);
        this.flavor1 = f1;
        this.flavor2 = f2;
    }

    public void setFlavor(String f) {
        super.setFlavor(f);
        this.flavor1 = f;
        this.flavor2 = f;
    }

    public void setFlavor(String f1, String f2) {
        super.setFlavor(f1);
        this.flavor1 = f1;
        this.flavor2 = f2;
    }

    public void addTopping(String t) {
        this.topping = t;
    }

    public String toString() {
        String buf = "double";

        buf += " " + super.toString();

        if (this.flavor1.equals(this.flavor2)) {
            buf += " x2";
        } else {
            buf += " and " + this.flavor2;
        }

        if (this.topping != null) {
            buf += " with " + this.topping;
        }

        return buf;
    }
}