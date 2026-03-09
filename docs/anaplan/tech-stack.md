# Anaplan - Tech Stack

Polaris Engine is known to use both Rust and C++ modules where i assume that,

- Storage and data structures are created in C++ to allow for leveraging eigen3 or other linear
  algebra dependencies to avoid "re-inventing the wheel"
- Rust would be used more for parallelization and orchestration of calculation jobs, as it is natively
  designed around thread safety (via the borrow checker)
    - Rust also provides the cargo ecosystem which allows for the much simpler management of projects
      in comparison to C++ which doesn't have a standard PM or build system (although CMake and make are often
      used)

## C++ - Eigen 3

Eigen is a high performance C++ linear algebra library, and is used by CERN as well as various other
scientific institutes and software packages.

### Why Eigen for Polaris

- **Sparse Matrix Support:** Native sparse matrix support and data structures for all formats (CSR, CSC)
- **Header-Only:** Requires less complexity from the build system
- **Performance:** Computations are natively optimized via
- **Memory Efficient:** Designed for large-scale linear algebra, and multi-dimensional objects
- **cuSPARSE Integration:** Depending on the calculation node type, cuSPARSE can be implemented to allow
  for GPU based linear algebra problem solving (Likely custom built)

### Provided Algorithms

- Direct Sparse `LL^T` and `LDL^T` Cholesky factorization
    - for Sparse Self-Adjoint Positive Definite Problems (SPD), for example:
        - Monte Carlo Simulations
        - Linear Equations
        - Non-linear Optimization
        - Data Imputation, this use case of Cholesky Factorization is good evidence for Eigen or a similiar
          C++ linear algebra package being used being used)
- Sparse LU Factorization
- Sparse QR Factorization
- Iterative Linear Solvers

### Rust Integration

Eigen can be leveraged from rust by creating a Foreign Function Interface binding to either an eigen wrapper
package or to Eigen itself (depending on the use case). This FFI can be approached by using one of the following methods:

- **CXX Crate**
- **bindgen**
- **CC Crate**

Where a typical wrapper pattern would involve:

- Implementing core data structures and operations in C++ leveraging Eigen
- Exposing a compatible API from C++
- Creating bindings to the Rust modules via one of the above FFI implementations
- Utilizing the rust bindings for orchestration and parallelization.

These packages can all be built by either the creation of a `build.rs` file with the appropriate linkers or
via the use of a system like CMake.

## Rust - Orchestration Layer

Rust handles job scheduling, parallelization, and the FFI bridge to C++:

The rust packages used by Anaplan are likely to focus on compilation, orchestration, and parallelization of
calculation nodes. It would also maintain the FFI to the Hyperblock C++ packages.

- Job scheduling manages calculation task queues and prioritizes cell evaluation order based on DAG
  dependencies
- Parallelization spawns worker threads for independent sub tree evaluation; borrow checker ensures no data
  races during concurrent access to shared model state
- FFI bridge to call into Eigen-based storage layer, where Rust owns orchestration/business logic and C++
  owns numerical operations

### Why Rust over C++

- Rust's borrow checker allows guarantees memory safety and prevents the creation of data race cases
- The rust package manager, `cargo`, provides excellent dependency management, testing systems, and a useful
  build system that largely allows for the minimization of CMake/make boilerplate
- Comparable performance to C++ and C
- Wide variety of third party packages that speed up development
- Interoperability with C++ and other languages via the usage of safe Foreign Function Interfaces (FFI's)

## Hyperblock Architecture

Hyperblock is Anaplan's proprietary multi-dimensional sparse array/tensor structure. It abstracts
dimensionality on top of 2D sparse storage.

```text
User Model (N dimensions)
    │
    ▼
Hyperblock Layer (dimension mapping, metadata)
    │
    ▼
Eigen Sparse Matrix (2D: rows × cols)
```

### Calculation Directed Acyclic Graph (DAG)

The Hyperblock system likely compiles its calculation dependency structures into a Directed Acyclic
Graph where nodes = calculated cells and edges = dependencies.
