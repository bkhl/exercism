class RnaTranscription {

    String transcribe(String dnaStrand) {
        StringBuffer result = new StringBuffer();
        for (char c : dnaStrand.toCharArray()) {
            switch (c) {
                case 'G':
                    result.append('C');
                    break;
                case 'C':
                    result.append('G');
                    break;
                case 'T':
                    result.append('A');
                    break;
                case 'A':
                    result.append('U');
                    break;
            }
        }
        return result.toString();
    }

}