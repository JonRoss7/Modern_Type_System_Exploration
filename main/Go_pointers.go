package main

import "fmt"

func main() {
    
    x := 10
    fmt.Println("Original value of x:", x)

    y := &x 
    
    *y = 20 
    
    fmt.Println("Modified value of x via pointer:", x)
}