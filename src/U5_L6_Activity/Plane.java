// @ignore Java(536871240)
// package U5_L6_Activity;
public class Plane {
  int location;

  public Plane() {
    this(0);
  }

  public Plane(int loc) {
    if (loc >= -2000 && loc <= 2000) {
      location = loc;
    } else {
      location = 0;
    }
  }

  public void upward() {
    location += 100;

    // clamp
    if (location > 2000) {
      location = 2000;
    }
  }

  public void downward() {
    location -= 100;

    // clamp
    if (location < -2000) {
      location = -2000;
    }
  }

  public int getLocation() {
    return location;
  }

  public String toString() {
    String s = "";

    for (int i = -2000; i < location; i += 100) {
      s += " ";
    }

    s += "@";

    return s;
  }

}
