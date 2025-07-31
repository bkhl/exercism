import java.util.HashSet;
import java.util.Set;

public class PangramChecker {

	public boolean isPangram(String input) {
		Set<Character> letters = new HashSet<>();

		for (char c : input.toCharArray()) {
			if (!Character.isAlphabetic(c)) {
				continue;
			}

			letters.add(Character.toLowerCase(c));
		}

		return letters.size() == 26;
	}
}