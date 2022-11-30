public class U5_L2_Activity_One {
    public static void timeOfDay(int hour) {
        // wrapping
        while (hour < 0 || hour > 24) {
            hour = 24 - hour; // -1 -> 23
            hour %= 24; // 25 -> 1
        }

        // 0 = "midnight"
        if (hour == 0) {
            System.out.println("midnight");
        }

        // 0..12 = "morning"
        else if (hour > 0 && hour < 12) {
            System.out.println("morning");
        }

        // 12 = "noon"
        else if (hour == 12) {
            System.out.println("noon");
        }

        // 12..18 = "afternoon"
        else if (hour > 12 && hour < 18) {
            System.out.println("afternoon");
        }

        // 18 = "dusk"
        else if (hour == 18) {
            System.out.println("dusk");
        }

        // 18..24 = "evening"
        else if (hour > 18 && hour < 24) {
            System.out.println("evening");
        }
    }
}
