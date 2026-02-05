
cmake_minimum_required(VERSION 3.5...3.25)
set(APP_EXE example)

project(${APP_EXE} C)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED ON)

add_executable(${APP_EXE} main.c)
# set_target_properties(${APP_EXE} PROPERTIES LINKER_LANGUAGE CXX)

target_link_libraries(${APP_EXE} kappa_c_wrap)

if(WIN32)
    # add_custom_command(TARGET ${APP_EXE} POST_BUILD
    #     COMMAND ${CMAKE_COMMAND} -E copy_if_different
    #     $<TARGET_FILE:kappa++::kappa++>
    #     $<TARGET_FILE_DIR:${APP_EXE}>
    #     COMMENT "Copying kappa++.dll to executable directory..."
    # )
    add_custom_command(TARGET ${APP_EXE} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
        $<TARGET_RUNTIME_DLLS:${APP_EXE}>
        $<TARGET_FILE_DIR:${APP_EXE}>
        COMMAND_EXPAND_LISTS
        COMMENT "Copying $<TARGET_RUNTIME_DLLS:${APP_EXE}> to ${APP_EXE}"
    )
    message(STATUS "Set dlls copying")
endif()
