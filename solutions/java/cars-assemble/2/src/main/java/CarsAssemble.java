public class CarsAssemble {
    private double successRate(int speed) {
        if (speed <= 4) return 1.0;
        if (speed <= 8) return .9;
        if (speed <= 9) return .8;
        else return .77;
    }

    public double productionRatePerHour(int speed) {
        return speed * 221 * successRate(speed);
    }

    public int workingItemsPerMinute(int speed) {
        return (int) (productionRatePerHour(speed) / 60);
    }
}
