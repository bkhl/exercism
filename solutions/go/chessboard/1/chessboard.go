package chessboard

// A type which stores if a square is occupied by a piece
type File [8]bool

// A type which should contain a map of eight Files, accessed with keys from "A" to "H"
type Chessboard map[string]File

func countInFile(file File) int {
	var result int
	for _, occupied := range file {
		if occupied {
			result++
		}
	}
	return result
}

// CountInFile returns how many squares are occupied in the chessboard,
// within the given file.
func CountInFile(cb Chessboard, file string) int {
	return countInFile(cb[file])
}

// CountInRank returns how many squares are occupied in the chessboard,
// within the given rank.
func CountInRank(cb Chessboard, rank int) int {
	var result int

	if rank < 1 || rank > 8 {
		return result
	}

	for _, file := range cb {
		if file[rank-1] {
			result++
		}
	}
	return result
}

// CountAll should count how many squares are present in the chessboard.
func CountAll(cb Chessboard) int {
	return len(cb) * 8
}

// CountOccupied returns how many squares are occupied in the chessboard.
func CountOccupied(cb Chessboard) int {
	var result int
	for _, file := range cb {
		result += countInFile(file)
	}
	return result
}
