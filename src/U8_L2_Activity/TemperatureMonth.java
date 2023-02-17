public class TemperatureMonth {
    private double[][] temperatures;

    public TemperatureMonth(double[][] t) {
        temperatures = t;
    }

    public double[] getMaxTempWeekly() {
        double[] weeks = new double[this.temperatures.length];
        for (int i = 0; i < this.temperatures.length; i++) {
            double max = Double.NEGATIVE_INFINITY;
            for (int j = 0; j < this.temperatures[i].length; j++) {
                if (this.temperatures[i][j] > max) {
                    max = this.temperatures[i][j];
                }
            }

            weeks[i] = max;
        }

        return weeks;
    }

    public double[] getMinTempWeekly() {
        double[] weeks = new double[this.temperatures.length];
        for (int i = 0; i < this.temperatures.length; i++) {
            double min = Double.POSITIVE_INFINITY;
            for (int j = 0; j < this.temperatures[i].length; j++) {
                if (this.temperatures[i][j] < min) {
                    min = this.temperatures[i][j];
                }
            }

            weeks[i] = min;
        }

        return weeks;
    }

    public double getRange() {
        double max = Double.NEGATIVE_INFINITY;
        double min = Double.POSITIVE_INFINITY;

        for (int i = 0; i < this.temperatures.length; i++) {
            for (int j = 0; j < this.temperatures[i].length; j++) {
                double value = this.temperatures[i][j];

                if (value > max) {
                    max = value;
                }

                if (value < min) {
                    min = value;
                }
            }
        }

        return max - min;
    }

    public double getCertainTemp(int week, int day) {
        return this.temperatures[week][day];
    }

}
