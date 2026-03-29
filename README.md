# Modern Type System Exploration

A multi-language learning repository that compares type-system behavior, pointers/references, and data modeling patterns across:

- Go
- Rust
- TypeScript
- Swift
- Mojo

## Repository Layout

```text
.
├── LICENSE.txt
├── README.md
├── main/
│   └── Go_pointers.go
├── part_Two/
│   ├── Go_Name_Equivalence.go
│   ├── TypeScript_Equivalence.ts
│   ├── TypeScript_Equivalence.js
│   ├── rust_pointers.rs
│   ├── rust_pointers            # compiled binary artifact
│   ├── rustTuplesStruct.rs
│   ├── rustTuplesStruct         # compiled binary artifact
│   └── swiftNamedTuples.swift
├── Practical_Tasks/
│   ├── Go_Associative_Array.go
│   ├── Rust_Enumeration.rs
│   ├── Rust_Enumeration         # compiled binary artifact
│   ├── Type_Script_Union.ts
│   └── Type_Script_Union.js
├── hello-world/
│   ├── Mojo_pointers.mojo
│   ├── pixi.toml
│   ├── pixi.lock
│   ├── .gitignore
│   └── .gitattributes
├── .swift-version
├── swiftly-1.1.1-x86_64.tar.gz
└── swiftly-x86_64.tar.gz
```

## What Each Example Demonstrates

### Core Example

- `main/Go_pointers.go`
	- Basic Go pointer usage (`&` and `*`) by mutating a value through its address.

### Part Two: Type Equivalence + Data Shape Comparisons

- `part_Two/Go_Name_Equivalence.go`
	- Named type equivalence in Go (`Car` vs `Bike`) and explicit conversion requirements.

- `part_Two/TypeScript_Equivalence.ts` (+ compiled `TypeScript_Equivalence.js`)
	- Structural typing in TypeScript, where compatible object shapes are assignable.

- `part_Two/rust_pointers.rs`
	- Rust mutable borrowing and scoped references for safe mutation.

- `part_Two/rustTuplesStruct.rs`
	- Tuple-based data vs named-field struct representation.

- `part_Two/swiftNamedTuples.swift`
	- Named tuples in Swift for lightweight structured values.

### Practical Tasks

- `Practical_Tasks/Go_Associative_Array.go`
	- Go maps as associative arrays and missing-key behavior (zero-value return).

- `Practical_Tasks/Rust_Enumeration.rs`
	- Rust enums with payload variants and `match` pattern handling.

- `Practical_Tasks/Type_Script_Union.ts` (+ compiled `Type_Script_Union.js`)
	- TypeScript union types with runtime narrowing (`typeof` and `Array.isArray`).

### Mojo Setup + Example

- `hello-world/Mojo_pointers.mojo`
	- Basic value manipulation in Mojo.

- `hello-world/pixi.toml`
	- Pixi workspace configuration for the Mojo environment.

## Running The Examples

From the repository root:

### Go

```bash
go run main/Go_pointers.go
go run part_Two/Go_Name_Equivalence.go
go run Practical_Tasks/Go_Associative_Array.go
```

### Rust

```bash
rustc part_Two/rust_pointers.rs -o part_Two/rust_pointers && ./part_Two/rust_pointers
rustc part_Two/rustTuplesStruct.rs -o part_Two/rustTuplesStruct && ./part_Two/rustTuplesStruct
rustc Practical_Tasks/Rust_Enumeration.rs -o Practical_Tasks/Rust_Enumeration && ./Practical_Tasks/Rust_Enumeration
```

### TypeScript

```bash
tsc part_Two/TypeScript_Equivalence.ts && node part_Two/TypeScript_Equivalence.js
tsc Practical_Tasks/Type_Script_Union.ts && node Practical_Tasks/Type_Script_Union.js
```

If JavaScript files are already generated, you can run them directly:

```bash
node part_Two/TypeScript_Equivalence.js
node Practical_Tasks/Type_Script_Union.js
```

### Swift

```bash
swift part_Two/swiftNamedTuples.swift
```

### Mojo (Pixi workspace)

```bash
cd hello-world
pixi run mojo Mojo_pointers.mojo
```