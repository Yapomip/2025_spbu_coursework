
set(CMAKE_CXX_STANDARD 17)

set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)

add_library(kappa_c_wrap STATIC wrap.cpp)

target_link_libraries(kappa_c_wrap PRIVATE kappa++::kappa++)

target_compile_definitions(kappa_c_wrap INTERFACE "KAPPA_RESOURCES_PATH=\"${KAPPA_RESOURCES_FILES_DIR}\"")
message(STATUS "KAPPA_RESOURCES_PATH=\"${KAPPA_RESOURCES_FILES_DIR}\"")

target_include_directories(kappa_c_wrap PUBLIC 
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>
    $<INSTALL_INTERFACE:include>
)

install(FILES "wrap.h" DESTINATION include)

install(
    TARGETS kappa_c_wrap 
    EXPORT kappa_c_wrapTargets
    LIBRARY DESTINATION lib 
    ARCHIVE DESTINATION lib 
    INCLUDES DESTINATION include
    RUNTIME DESTINATION lib
)

install(
    EXPORT kappa_c_wrapTargets
    FILE kappa_c_wrapTargets.cmake
    DESTINATION cmake
)
