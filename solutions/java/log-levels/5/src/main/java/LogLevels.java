import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class LogLevels {
    private static Pattern logLinePattern = Pattern.compile("^\\[(?<level>.+)\\]:\\s*(?<message>.*?)\\s*$");

    public static String message(String logLine) {
        int start = logLine.indexOf(":") + 1;
        return logLine.substring(start).trim();
    }

    public static String logLevel(String logLine) {
        int start = logLine.indexOf("[") + 1;
        int end = logLine.indexOf("]", start);
        return logLine.substring(start, end).toLowerCase();
    }

    public static String reformat(String logLine) {
        return message(logLine).concat(" (").concat(logLevel(logLine)).concat( ")");
    }
}
