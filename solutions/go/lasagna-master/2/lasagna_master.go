package lasagna

const (
	defaultMinutesPerLayer = 2
	noodleQuantityPerLayer = 50
	sauceQuantityPerLayer  = 0.2
	normalRecipeServings   = 2
)

// PreparationTime returns the preparatation time given the layers of the
// lasagna.
func PreparationTime(layers []string, minutesPerLayer int) int {
	if minutesPerLayer == 0 {
		minutesPerLayer = defaultMinutesPerLayer
	}
	return len(layers) * minutesPerLayer
}

// Quantities returns the amount of noodles and sauce needed given the layers of
// the lasagna.
func Quantities(layers []string) (int, float64) {
	noodleLayers := 0
	sauceLayers := 0.

	for _, layer := range layers {
		switch layer {
		case "noodles":
			noodleLayers++
		case "sauce":
			sauceLayers++
		}
	}

	return noodleLayers * noodleQuantityPerLayer, float64(sauceLayers) * sauceQuantityPerLayer
}

// AddSecretIngredient injects the secret ingredient into the ingredient list.
func AddSecretIngredient(friendsList []string, myList []string) {
	myList[len(myList)-1] = friendsList[len(friendsList)-1]
}

// ScaleRecipe calculates the ingredients needed given a number of portions.
func ScaleRecipe(quantities []float64, portions int) []float64 {
	var scaledQuantities []float64
	for _, quantity := range quantities {
		scaledQuantities = append(scaledQuantities, quantity*float64(portions)/normalRecipeServings)
	}
	return scaledQuantities
}
