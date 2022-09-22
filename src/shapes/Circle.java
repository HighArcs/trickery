package shapes;

public class Circle extends java.lang.Object {
    private double radius = 1.0f;

    public Circle() {
    }

    public Circle(double radius) {
        this.radius = radius;
    }

    public double getArea() {
        return Math.PI * Math.pow(this.getRadius(), 2.0f);
    }

    public double getCircumference() {
        return 2.0f * Math.PI * this.getRadius();
    }

    public double getRadius() {
        return this.radius;
    }

    public void setRadius(double radius) {
        this.radius = radius;
    }

    public String toString() {
        return "circle with radius " + this.getRadius();
    }
}
