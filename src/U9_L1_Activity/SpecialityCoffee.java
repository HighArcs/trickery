public class SpecialityCoffee extends Coffee {
    public String flavor;
    public SpecialityCoffee() {
        super("small", false, 1, "latte");
        this.flavor = "vanilla";
    }

    public SpecialityCoffee(String size, String type, String flavor) {
        super(size, false, 1, type);
        this.flavor = flavor;
    }

    public SpecialityCoffee(String size, boolean isSkinny, int shots, String type, String flavor) {
        super(size, isSkinny, shots, type);
        this.flavor = flavor;
    }

    public String toString() {
        return super.toString() + " with " + this.flavor;
    }
}
