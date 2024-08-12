This Rust project outlines how to utilize the TFHE library to perform secure homomorphic computation with adjustable cryptographic parameters. The code demonstrates how to conduct various arithmetic and logical operations on encrypted data while maintaining at least 128-bit security and low error probability throughout the process.

## Features
- **Fully Homomorphic Encryption (FHE):** Perform computations directly on encrypted data without decrypting it. 
- **Customizable Cryptographic Parameters:** Set cryptographic parameters to fulfill specific security requirements.
- **Arithmetic Operations:** Addition, subtraction, multiplication, division, remainder, and bitwise operations on encrypted 8-bit unsigned integers. 
- **Logical Operations:** AND, OR, XOR, negation, and bitwise shifts on encrypted values. 
- **Comparison Operations:** Equality, inequality, greater-than, and less-than comparisons on encrypted data.


## Requirements
- **Rust:** Ensure you have the latest stable version of Rust installed.

>[!NOTE]
>Rust version: a minimum Rust version of 1.73 is required to compile TFHE-rs.

- **TFHE Library:** The project depends on the TFHE library for fully homomorphic encryption. Ensure the library is included in your `Cargo.toml` file. 

``` 
#To include library run:
cargo add tfhe

#Alternatively paste the line below in 'Cargo.toml' 
#For x86_64 machine running a Unix-like OS:

tfhe = { version = "0.7.2", features = [ "boolean", "shortint", "integer", "x86_64-unix" ] }
#For ARM machine running a Unix-like OS:

tfhe = { version = "0.7.2", features = [ "boolean", "shortint", "integer", "aarch64-unix" ] }
#For x86_64 machines with the rdseed instruction running Windows:

tfhe = { version = "*", features = ["boolean", "shortint", "integer", "x86_64"] }

#ensure to build cargo after adding the tfhe library
cargo run build
```
## Installation
```
git clone  https://github.com/cypriansakwa/Secure_Homomorphic_Computation_Using_TFHE_with_Customizable_Cryptographic_Parameters.git
```
```
cd Secure_Homomorphic_Computation_Using_TFHE_with_Customizable_Cryptographic_Parameters
```

## Main Steps
- **Configuration and Key Generation:** The ConfigBuilder is used to generate a configuration with the default cryptographic parameters. This configuration generates client and server keys.

- **Encryption:** Clear values are encrypted using the client key to keep them secure during the computation.

- **Homomorphic Computation:** Various arithmetic logical, and comparison operations are carried out directly on the encrypted data.

- **Decryption:** The results of the computations are decrypted and verified against expected clear values.

## Code Structure
- main.rs: Contains the core logic for configuring, encrypting, computing, and verifying results.
- optimized_operations.rs: Implements optimized arithmetic and comparison operations on encrypted values.
   
## Run code
>[!TIP]
> Performance: for optimal performance, it is highly recommended to run code that uses TFHE-rs in release mode with cargo's --release flag.
>```
cargo -- run release
>```