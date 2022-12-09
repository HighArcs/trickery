// @ignore Java(536871240)
// package U5_L7_Activity;

public class Rectangle extends java.lang.Object {
  double bs;
  double ht;

  public Rectangle() {
    this(1, 1);
  }

  public Rectangle(double base, double height) {
    if (base < 0) {
      base = 1;
    }
    if (height < 0) {
      height = 1;
    }

    this.bs = base;
    this.ht = height;
  }

  public boolean equals(Rectangle other) {
    return (other.getBase() == this.getBase() && other.getHeight() == this.getHeight());
  }

  public double getArea() {
    return getBase() * getHeight();
  }

  public double getBase() {
    return bs;
  }

  public double getDiagonal() {
    return Math.sqrt(Math.pow(bs, 2) + Math.pow(ht, 2));
  }

  public double getHeight() {
    return this.ht;
  }

  public double getPerimeter() {
    return 2 * this.getBase() + 2 * this.getHeight();
  }

  public void setBase(double bs) {
    if (bs > 0) {
      this.bs = bs;
    }
  }

  public void setHeight(double ht) {
    if (ht > 0) {
      this.ht = ht;
    }
  }

  public String toString() {
    return "rectangle with base " + this.getBase() + ", height " + this.getHeight();
  }
}