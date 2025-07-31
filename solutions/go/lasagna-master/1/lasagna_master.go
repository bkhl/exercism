package lasagna

// PreparationTime returns the preparatation time given the layers of the
// lasagna.
func PreparationTime(layers []string, minutesPerLayer int) int {
	if minutesPerLayer == 0 {
		minutesPerLayer = 2
	}
	return len(layers) * minutesPerLayer
}

// Quantities returns the amount of noodles and sauce needed given the layers of
// the lasagna.
func Quantities(layers []string) (int, float64) {
	noodles := 0
	sauce := 0.

	for _, layer := range layers {
		switch layer {
		case "noodles":
			noodles += 50
		case "sauce":
			sauce += 0.2
		}
	}

	return noodles, sauce
}

// AddSecretIngredient injects the secret ingredient into the ingredient list.
func AddSecretIngredient(friendsList []string, myList []string) {
	myList[len(myList)-1] = friendsList[len(friendsList)-1]
}

// ScaleRecipe calculates the ingredients needed given a number of portions.
func ScaleRecipe(quantities []float64, portions int) []float64 {
	scaled_quantities := make([]float64, len(quantities))
	for i, quantity := range quantities {
		scaled_quantities[i] = quantity * float64(portions) / 2
	}
	return scaled_quantities
}
