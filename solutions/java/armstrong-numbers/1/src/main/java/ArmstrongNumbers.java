import java.util.Stack;

class ArmstrongNumbers {

    boolean isArmstrongNumber(int numberToCheck) {
        int i = numberToCheck;
        Stack<Integer> digits = new Stack<>();

        while (true) {
            if (i < 10) {
                digits.add(i);
                break;
            }
            digits.add(i % 10);
            i = i / 10;
        }

        int number_of_digits = digits.size();
        int sum = 0;
        for (Integer digit : digits) {
            sum += java.lang.Math.pow(digit, number_of_digits);
        }

        return sum == numberToCheck;
    }
}