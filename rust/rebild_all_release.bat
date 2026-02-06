cd kappa
cmake -B build -D CMAKE_BUILD_TYPE=Release
cmake --build build --config Release --parallel 8
cmake --install build --prefix install
cd ..
cd kappa_c_wrap
cmake -B build -D CMAKE_BUILD_TYPE=Release -D KAPPA_C_WRAP_EXAMPLE=ON
cmake --build build --config Release --parallel 8
cmake --install build --prefix install
.\build\example\example.exe
cd ..
cd kappa_rust
cargo r --example 1 --release