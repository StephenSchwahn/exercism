// Package weather privides the forcast in a particular region.
package weather

var (
	// CurrentCondition is a string that represents the current condition at a particular
    // location.
    CurrentCondition string
    // CurrentLocation is a string that contains the currently selected region.
	CurrentLocation  string
)

// Forecast returns a forcast string given a city and condition as inputs.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
