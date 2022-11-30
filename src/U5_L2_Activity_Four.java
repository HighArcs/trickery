public class U5_L2_Activity_Four {
    public static void coinConverter(int pennies) {
        final int dollars = pennies / 100;
        pennies -= dollars * 100; // set pennies to the left over amount

        final int quarters = pennies / 25;
        pennies -= quarters * 25;

        final int dimes = pennies / 10;
        pennies -= dimes * 10;

        final int nickels = pennies / 5;
        pennies -= nickels * 5;

        System.out.println("Dollar Bills: " + dollars);
        System.out.println("Quarters: " + quarters);
        System.out.println("Dimes: " + dimes);
        System.out.println("Nickels: " + nickels);
        System.out.println("Pennies: " + pennies);
    }
}
