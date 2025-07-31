import java.util.Map;

class SqueakyClean {
    static Map<Character, Character> map = Map.ofEntries(
            Map.entry(' ', '_'),
            Map.entry('0', 'o'),
            Map.entry('1', 'l'),
            Map.entry('3', 'e'),
            Map.entry('4', 'a'),
            Map.entry('7', 't')
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
