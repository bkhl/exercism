
class BirdWatcher {
    private final int[] birdsPerDay;

    public BirdWatcher(int[] birdsPerDay) {
        this.birdsPerDay = birdsPerDay.clone();
    }

    public int[] getLastWeek() {
        return this.birdsPerDay;
    }

    public int getToday() {
        return this.birdsPerDay[6];
    }

    public void incrementTodaysCount() {
        this.birdsPerDay[6]++;
    }

    public boolean hasDayWithoutBirds() {
        for (int i : (this.birdsPerDay)) {
            if (i == 0) return true;
        }
        return false;
    }

    public int getCountForFirstDays(int numberOfDays) {
        int result = 0;

        for (int i = 0; i < numberOfDays && i < 7; i++) {
            result += this.birdsPerDay[i];
        }

        return result;
    }

    public int getBusyDays() {
        int result = 0;

        for (int i: (this.birdsPerDay)) {
            if (i >= 5) result++;
        }

        return result;
    }
}
