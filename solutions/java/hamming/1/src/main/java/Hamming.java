class Hamming {

	private String leftStrand;
	private String rightStrand;

	Hamming(String leftStrand, String rightStrand) {
		if (leftStrand.length() != rightStrand.length()) {
			throw new IllegalArgumentException("leftStrand and rightStrand must be of equal length.");
		}

		this.leftStrand = leftStrand;
		this.rightStrand = rightStrand;
	}

	int getHammingDistance() {
		int distance = 0;

		int strandLength = this.leftStrand.length();
		for (int i = 0; i < strandLength; i++) {
			if (this.leftStrand.charAt(i) != this.rightStrand.charAt(i)) {
				distance++;
			}
		}

		return distance;
	}

}