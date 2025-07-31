class ReverseString {
    String reverse(String inputString) {
        StringBuffer stringBuffer = new StringBuffer(inputString);
        return stringBuffer.reverse().toString();
    }
}