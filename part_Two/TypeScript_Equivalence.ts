interface Car {
    wheels: number;
}

interface Bike {
    wheels: number;
}

let myCar: Car = { wheels: 4 };

let myBike: Bike = myCar; 

console.log(`Car wheels: ${myCar.wheels}, Bike wheels: ${myBike.wheels}`);
console.log("Status: Direct assignment successful! TypeScript only checks the structure.");