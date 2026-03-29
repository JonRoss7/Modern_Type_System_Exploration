package main

import "fmt"

func main() {
    students := map[int]string{
        101: "Alice",
        102: "Bob",
    }

    missingID := 999
    name := students[missingID]

    fmt.Printf("Accessing missing ID %d returns: %q\n", missingID, name)
    fmt.Println("Observation: Go does not throw an error. It returns the 'zero value' for the type, which is an empty string.")
}