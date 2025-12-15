
// my fork of https://github.com/lkampoli/kappa
// https://github.com/Yapomip/kappa
kappa:
    library on c++
    calculate some physics coeficient

kappa_c_wrap:
    library on c++ with c headers
    c interface for kappa
    (not all kappa function wrapped)

kappa_wrapper:
    library on rust
    connect to kappa_c_wrap
    interface for kappa on rust

ns:
    executable on rust
    build ml to calculate some physics coeficient

run:

sudo apt-get install libopenblas-dev libarmadillo-dev libyaml-cpp-dev 
cargo r --release --features "wgpu"