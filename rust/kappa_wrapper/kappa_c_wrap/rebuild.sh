#!/bin/bash
rm -rf build
mkdir build
cd build
cmake .. -DKAPPA_C_WRAPPER_EXEMPLE=ON
cmake --build . 