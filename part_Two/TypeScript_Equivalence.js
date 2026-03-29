"use strict";
let myCar = { wheels: 4 };
let myBike = myCar;
console.log(`Car wheels: ${myCar.wheels}, Bike wheels: ${myBike.wheels}`);
console.log("Status: Direct assignment successful! TypeScript only checks the structure.");
