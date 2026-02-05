
cmake_minimum_required(VERSION 3.5...3.25)
project(dll_collector C)

message(STATUS "KAPPA_C_WRAP_DIR ${KAPPA_C_WRAP_DIR}")
message(STATUS "COPY_TO_DIR ${COPY_TO_DIR}")
if(NOT DEFINED KAPPA_C_WRAP_DIR OR NOT DEFINED COPY_TO_DIR)
    message(FATAL_ERROR "NOT OK")
endif()

message("Try to find package")
find_package(kappa_c_wrap QUIET HINTS ${KAPPA_C_WRAP_DIR})
if(NOT TARGET kappa_c_wrap)
    message("Try to add subdirectory")
    add_subdirectory(${KAPPA_C_WRAP_DIR} ${COPY_TO_DIR}/build_KAPPA_C_WRAP)
endif()

if(NOT TARGET kappa_c_wrap)
    message(FATAL_ERROR "NOT OK")
endif()

file(WRITE "${CMAKE_CURRENT_BINARY_DIR}/dummy.cpp" "int main() {return 0;} ")
add_executable(dll_collector "${CMAKE_CURRENT_BINARY_DIR}/dummy.cpp")
target_link_libraries(dll_collector kappa_c_wrap)

# add_library(dll_collector MODULE)
# target_link_libraries(dll_collector INTERFACE kappa_c_wrap)

add_custom_command(
    TARGET dll_collector POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
    $<TARGET_RUNTIME_DLLS:dll_collector>
    ${COPY_TO_DIR}
    COMMAND_EXPAND_LISTS
    COMMENT "Copying $<TARGET_RUNTIME_DLLS:dll_collector> to ${COPY_TO_DIR}"
)
message(STATUS "Set dlls copying")