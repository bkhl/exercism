import java.util.Map;

class SqueakyClean {
    static Map<Character, Character> map = Map.of(
            ' ', '_',
            '0', 'o',
            '1', 'l',
            '3', 'e',
            '4', 'a',
            '7', 't'
    );

    static String clean(String identifier) {
        StringBuilder r = new StringBuilder();
        boolean newWord = false;

        for (char c : identifier.toCharArray()) {
            if (map.containsKey(c)) {
                r.append(map.get(c));
            } else if (Character.isLetter(c)) {
                if (newWord) {
                    r.append(Character.toUpperCase(c));
                    newWord = false;
                } else {
                    r.append(c);
                }
            } else if (c == '-') {
                newWord = true;
            }
        }

        return r.toString();
    }
}
