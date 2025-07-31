import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class LogLevels {
    private static Pattern logLinePattern = Pattern.compile("^\\[(?<level>.+)\\]:\\s*(?<message>.*?)\\s*$");

    private static Matcher match(String logLine) {
        Matcher matcher = logLinePattern.matcher(logLine);
        matcher.matches();
        return matcher;
    }

    public static String message(String logLine) {
        Matcher matcher = match(logLine);
        return matcher.group("message");
    }

    public static String logLevel(String logLine) {
        Matcher matcher = match(logLine);
        return matcher.group("level").toLowerCase();
    }

    public static String reformat(String logLine) {
        Matcher matcher = match(logLine);
        return message(logLine).concat(" (").concat(logLevel(logLine).toLowerCase()).concat( ")");
    }
}
