
add_library(kappa_c_wrap STATIC wrap.cpp)

target_link_libraries(kappa_c_wrap PRIVATE kappa++)

set_target_properties(${APP_EXE} PROPERTIES LINKER_LANGUAGE CXX)
# target_link_libraries(kappa_c_wrap ${YAML_CPP_LIBRARIES})

target_include_directories(kappa_c_wrap PUBLIC 
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>
    $<INSTALL_INTERFACE:include>
)

install(DIRECTORY .
    DESTINATION include/
    FILES_MATCHING PATTERN "*.h"
)

install(TARGETS kappa_c_wrap DESTINATION lib EXPORT kappa_c_wrap_targets)

install(EXPORT kappa_c_wrap_targets
    FILE kappacwrapConfig.cmake
    DESTINATION lib/cmake
)
