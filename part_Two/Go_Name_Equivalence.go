package main

import "fmt"

type Car struct {
    Wheels int
}

type Bike struct {
    Wheels int
}

func main() {
    
    myCar := Car{Wheels: 4}
    
    var myBike Bike = Bike(myCar)
    
    fmt.Printf("Car wheels: %d, Bike wheels: %d\n", myCar.Wheels, myBike.Wheels)
    fmt.Println("Status: Direct assignment fails. Explicit conversion is required because Go checks the type name.")
}