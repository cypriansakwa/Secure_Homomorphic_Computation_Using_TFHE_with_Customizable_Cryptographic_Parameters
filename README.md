This Rust project outlines how to utilize the TFHE library to perform secure homomorphic computation with adjustable cryptographic parameters. The code demonstrates how to conduct various arithmetic and logical operations on encrypted data while maintaining at least 128-bit security and low error probability throughout the process.

## Features
-**Fully Homomorphic Encryption (FHE):** Perform computations directly on encrypted data without decrypting it. 
- **Customizable Cryptographic Parameters:** Set cryptographic parameters to fulfill specific security requirements.
 - **Arithmetic Operations:** Addition, subtraction, multiplication, division, remainder, and bitwise operations on encrypted 8-bit unsigned integers. 
- **Logical Operations:** AND, OR, XOR, negation, and bitwise shifts on encrypted values. 
- **Comparison Operations:** Equality, inequality, greater-than, and less-than comparisons on encrypted data.


## Requirements
- **Rust:** Ensure you have the latest stable version of Rust installed. 
- **TFHE Library:** The project depends on the TFHE library for fully homomorphic encryption. Ensure the library is included in your `Cargo.toml` file. 
## Installation
## Main Steps
-**Configuration and Key Generation:** The ConfigBuilder is used to generate a configuration with the default cryptographic parameters. This configuration generates client and server keys.

-**Encryption:** Clear values are encrypted using the client key to keep them secure during the computation.

-**Homomorphic Computation:** Various arithmetic logical, and comparison operations are carried out directly on the encrypted data.

-**Decryption:** The results of the computations are decrypted and verified against expected clear values.

## Code Structure
1. main.rs: Contains the core logic for configuring, encrypting, computing, and verifying results.
2. optimized_operations.rs: Implements optimized arithmetic and comparison operations on encrypted values.
   
git clone  https://github.com/cypriansakwa/Secure_Homomorphic_Computation_Using_TFHE_with_Customizable_Cryptographic_Parameters.git
cd Secure_Homomorphic_Computation_Using_TFHE_with_Customizable_Cryptographic_Parameters
