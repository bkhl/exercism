// Package weather provides weather forecasts given a location and current
// weather conditions.
package weather

// CurrentCondition holds the condition at the last provided forecast.
var CurrentCondition string

// CurrentLocation holds the location of the last provided forecast.
var CurrentLocation string

// Forecast returns a forecast, given a location and weather condition, and
// stores the provided values.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
