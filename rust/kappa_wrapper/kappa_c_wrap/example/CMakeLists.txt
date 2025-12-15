

set(APP_EXE example)

add_executable(${APP_EXE} main.cpp)
set_target_properties(${APP_EXE} PROPERTIES LINKER_LANGUAGE CXX)

target_link_libraries(${APP_EXE} kappa_c_wrap)

