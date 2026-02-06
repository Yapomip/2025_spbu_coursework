cd kappa
cmake -B build -D CMAKE_BUILD_TYPE=Debug
cmake --build build --config Debug --parallel 8
cmake --install build --prefix install
cd ..
cd kappa_c_wrap
cmake -B build -D CMAKE_BUILD_TYPE=Debug -D KAPPA_C_WRAP_EXAMPLE=ON
cmake --build build --config Debug --parallel 8
cmake --install build --prefix install
.\build\example\example.exe
cd ..
cd kappa_rust
cargo r --example 1
