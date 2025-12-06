# Shadow OT Client CMakeLists.txt
# Modern Open Tibia client for macOS/Linux/Windows

cmake_minimum_required(VERSION 3.20)

project(shadow-client
    VERSION 1.0.0
    DESCRIPTION "Shadow OT Client - Modern Open Tibia Client"
    LANGUAGES CXX
)

# C++20 standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Build options
option(SHADOW_ENABLE_BLOCKCHAIN "Enable blockchain integration" ON)
option(SHADOW_ENABLE_ENCRYPTION "Enable protocol encryption" ON)

# Platform detection
if(APPLE)
    set(SHADOW_PLATFORM "macos")
elseif(WIN32)
    set(SHADOW_PLATFORM "windows")
elseif(UNIX)
    set(SHADOW_PLATFORM "linux")
endif()

message(STATUS "Shadow OT Client v${PROJECT_VERSION}")
message(STATUS "Target platform: ${SHADOW_PLATFORM}")
message(STATUS "C++ Standard: ${CMAKE_CXX_STANDARD}")

# Find packages with proper paths for Homebrew on macOS
if(APPLE)
    # Add Homebrew paths
    set(CMAKE_PREFIX_PATH
        "/opt/homebrew"
        "/opt/homebrew/opt/lua"
        "/opt/homebrew/opt/luajit"
        "/opt/homebrew/opt/openal-soft"
        "/opt/homebrew/opt/physfs"
        "/opt/homebrew/opt/zlib"
        "/opt/homebrew/opt/openssl@3"
        "/opt/homebrew/opt/glfw"
        "/opt/homebrew/opt/glew"
        ${CMAKE_PREFIX_PATH}
    )

    set(OPENAL_INCLUDE_DIR "/opt/homebrew/opt/openal-soft/include")
    set(OPENAL_LIBRARY "/opt/homebrew/opt/openal-soft/lib/libopenal.dylib")
endif()

find_package(OpenGL REQUIRED)
find_package(GLEW REQUIRED)
find_package(glfw3 REQUIRED)
find_package(ZLIB REQUIRED)
find_package(OpenSSL REQUIRED)
find_package(Threads REQUIRED)

# Find Lua (try LuaJIT first, then regular Lua)
find_package(PkgConfig QUIET)
if(PKG_CONFIG_FOUND)
    pkg_check_modules(LUA QUIET luajit)
    if(NOT LUA_FOUND)
        pkg_check_modules(LUA QUIET lua)
    endif()
endif()

if(NOT LUA_FOUND)
    find_package(Lua QUIET)
    if(LUA_FOUND)
        set(LUA_INCLUDE_DIRS ${LUA_INCLUDE_DIR})
        set(LUA_LIBRARIES ${LUA_LIBRARIES})
    else()
        # Manual fallback for macOS Homebrew
        if(APPLE)
            set(LUA_INCLUDE_DIRS "/opt/homebrew/include/lua")
            set(LUA_LIBRARIES "/opt/homebrew/lib/liblua.dylib")
            if(NOT EXISTS "${LUA_LIBRARIES}")
                set(LUA_INCLUDE_DIRS "/opt/homebrew/include/luajit-2.1")
                set(LUA_LIBRARIES "/opt/homebrew/lib/libluajit-5.1.dylib")
            endif()
        endif()
    endif()
endif()

# Find PhysFS
find_library(PHYSFS_LIBRARY physfs PATHS /opt/homebrew/lib)
find_path(PHYSFS_INCLUDE_DIR physfs.h PATHS /opt/homebrew/include)

# Find OpenAL
if(NOT OPENAL_LIBRARY)
    find_library(OPENAL_LIBRARY openal OpenAL PATHS /opt/homebrew/opt/openal-soft/lib)
    find_path(OPENAL_INCLUDE_DIR al.h PATH_SUFFIXES AL OpenAL PATHS /opt/homebrew/opt/openal-soft/include)
endif()

message(STATUS "Lua: ${LUA_LIBRARIES}")
message(STATUS "OpenAL: ${OPENAL_LIBRARY}")
message(STATUS "PhysFS: ${PHYSFS_LIBRARY}")

# Source files - only include files that exist
set(SHADOW_SOURCES
    src/main.cpp

    # Framework Core
    src/framework/core/application.cpp
    src/framework/core/eventdispatcher.cpp
    src/framework/core/configmanager.cpp
    src/framework/core/resourcemanager.cpp

    # Framework Graphics
    src/framework/graphics/graphics.cpp

    # Framework Input
    src/framework/input/inputmanager.cpp

    # Framework Network
    src/framework/net/protocol.cpp
    src/framework/net/connection.cpp

    # Framework Platform
    src/framework/platform/platform.cpp

    # Framework Lua
    src/framework/luaengine/luainterface.cpp

    # Framework Sound
    src/framework/sound/soundmanager.cpp

    # Framework UI
    src/framework/ui/uiwidget.cpp
    src/framework/ui/uilabel.cpp
    src/framework/ui/uibutton.cpp
    src/framework/ui/uitextinput.cpp
    src/framework/ui/uiwindow.cpp
    src/framework/ui/uiscrollarea.cpp
    src/framework/ui/uiprogressbar.cpp
    src/framework/ui/uicombobox.cpp
    src/framework/ui/uimanager.cpp

    # Client
    src/client/thing.cpp
    src/client/item.cpp
    src/client/thingtype.cpp
    src/client/creature.cpp
    src/client/player.cpp
    src/client/localplayer.cpp
    src/client/tile.cpp
    src/client/map.cpp
    src/client/mapview.cpp
    src/client/container.cpp
    src/client/effect.cpp
    src/client/missile.cpp
    src/client/game.cpp
    src/client/protocolgame.cpp
    src/client/luabindings.cpp

    # Shadow Extensions
    src/shadow/realms/realmmanager.cpp
    src/shadow/blockchain/wallet.cpp
)

# Main executable
add_executable(shadow-client ${SHADOW_SOURCES})

# Include directories
target_include_directories(shadow-client PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_SOURCE_DIR}/src/framework
    ${CMAKE_SOURCE_DIR}/src/client
    ${CMAKE_SOURCE_DIR}/src/shadow
    ${LUA_INCLUDE_DIRS}
    ${OPENAL_INCLUDE_DIR}
    ${PHYSFS_INCLUDE_DIR}
    ${OPENSSL_INCLUDE_DIR}
)

# Link libraries
target_link_libraries(shadow-client PRIVATE
    OpenGL::GL
    GLEW::GLEW
    glfw
    ${LUA_LIBRARIES}
    ${OPENAL_LIBRARY}
    ${PHYSFS_LIBRARY}
    ZLIB::ZLIB
    OpenSSL::SSL
    OpenSSL::Crypto
    Threads::Threads
)

# macOS-specific frameworks
if(APPLE)
    target_link_libraries(shadow-client PRIVATE
        "-framework Cocoa"
        "-framework IOKit"
        "-framework CoreFoundation"
        "-framework CoreAudio"
        "-framework AudioToolbox"
    )
endif()

# Compile definitions
target_compile_definitions(shadow-client PRIVATE
    SHADOW_VERSION="${PROJECT_VERSION}"
    SHADOW_PLATFORM="${SHADOW_PLATFORM}"
    $<$<BOOL:${SHADOW_ENABLE_BLOCKCHAIN}>:SHADOW_BLOCKCHAIN_ENABLED>
    $<$<BOOL:${SHADOW_ENABLE_ENCRYPTION}>:SHADOW_ENCRYPTION_ENABLED>
    $<$<CONFIG:Debug>:SHADOW_DEBUG>
)

# Compiler options
if(CMAKE_CXX_COMPILER_ID MATCHES "Clang|GNU")
    target_compile_options(shadow-client PRIVATE
        -Wall -Wextra
        -Wno-unused-parameter
        -Wno-missing-field-initializers
        $<$<CONFIG:Release>:-O3>
        $<$<CONFIG:Debug>:-g -O0>
    )
endif()

# Install
install(TARGETS shadow-client RUNTIME DESTINATION bin)
install(DIRECTORY modules/ DESTINATION share/shadow-client/modules OPTIONAL)
install(DIRECTORY data/ DESTINATION share/shadow-client/data OPTIONAL)
