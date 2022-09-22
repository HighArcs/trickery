package shapes;

public class Rectangle extends java.lang.Object {
    private double length = 1.0f;
    private double width = 1.0f;

    public Rectangle() {
    }

    public Rectangle(double size) {
        this.length = size;
        this.width = size;
    }

    public Rectangle(double length, double width) {
        this.length = length;
        this.width = width;
    }

    public double getArea() {
        return this.getLength() * this.getWidth();
    }

    public double getPerimeter() {
        return (2 * this.getLength()) + (2 * this.getWidth());
    }

    public double getLength() {
        return this.length;
    }

    public double getWidth() {
        return this.width;
    }

    public void setLength(double length) {
        this.length = length;
    }

    public void setWidth(double width) {
        this.width = width;
    }

    public String toString() {
        return "rectangle with length " + this.getLength() + ", width " + this.getWidth();
    }
}
