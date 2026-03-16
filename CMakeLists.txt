# CMakeLists.txt for Avox

cmake_minimum_required(VERSION 3.10)

project(Avox CXX)

# Find OpenSSL
find_package(OpenSSL REQUIRED)

# Include post-quantum cryptography library
find_package(PostQuantumCrypto REQUIRED)

# Set up Rust FFI
include(ExternalProject)
ExternalProject_Add(
    rust_ffi
    PREFIX ${CMAKE_BINARY_DIR}/rust_ffi
    GIT_REPOSITORY https://github.com/your-rust-ffi-repo.git
    # Add more needed options depending on the Rust setup
)

# Add C++ executable
add_executable(Avox src/main.cpp)

# Link OpenSSL
target_link_libraries(Avox OpenSSL::SSL OpenSSL::Crypto PostQuantumCrypto)

# Include Rust FFI
add_dependencies(Avox rust_ffi)