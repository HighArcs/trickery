
public class tests {

  public static void main(String[] args) {

    Subtracter a = new Subtracter(100);

    Subtracter b = new Subtracter(25);

    Subtracter c = b;

    c.decrease(50);

    System.out.println(a.getValue() + " " + b.getValue() + " " + c.getValue());
  }

}

class Subtracter

{

  private int value;

  public Subtracter(int n)

  {

    value = n;

  }

  public void decrease(int less)

  {

    value = value - less;

  }

  public int getValue()

  {

    return value;

  }

}
