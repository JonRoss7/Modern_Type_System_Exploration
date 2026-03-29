function printLength(input: string | Array<string>): void {
    if (typeof input === "string") {
        console.log(`String length: ${input.length} characters`);
    } 
    else if (Array.isArray(input)) {
        console.log(`Array length: ${input.length} items`);
    }
}

printLength("Hello World");
printLength(["Apple", "Banana", "Cherry"]);