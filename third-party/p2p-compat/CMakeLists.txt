cmake_minimum_required(VERSION 3.10)

project(p2p-compat CXX)

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# miniupnpc introduce undesired main() functions, rename them:
add_compile_options(-Dmain=main_off)

if(NOT MSVC)
  set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -g")
  set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O3")
else()
  set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} /O2")
    add_definitions(-DNOMINMAX)
    add_definitions(-D_WIN32_WINNT=0x0601)
    add_definitions(-D_SILENCE_CXX17_ALLOCATOR_VOID_DEPRECATION_WARNING)
    add_definitions(-D_SILENCE_CXX17_ITERATOR_BASE_CLASS_DEPRECATION_WARNING)
    add_definitions(-D_ENABLE_EXTENDED_ALIGNED_STORAGE)
    add_definitions(-D_CRT_SECURE_NO_WARNINGS)
    add_definitions(-D_WINSOCK_DEPRECATED_NO_WARNINGS)
endif()

set(Boost_USE_MULTITHREADED ON)
set(Boost_USE_STATIC_LIBS ON)

if(NOT APPLE)
  set(Boost_USE_STATIC_RUNTIME ON)
endif()

find_package(Boost 1.66 REQUIRED COMPONENTS system)
find_package(Threads)

add_subdirectory(p2p)

add_library(${PROJECT_NAME} STATIC
  p2p-compat.h
  p2p-compat.cpp)

include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/p2p/third-party/easyloggingpp/src
    ${CMAKE_CURRENT_SOURCE_DIR}/p2p/third-party/bitcoin_utils
    ${CMAKE_CURRENT_SOURCE_DIR}/p2p/include)

target_include_directories(${PROJECT_NAME} PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

target_link_libraries(${PROJECT_NAME}
  PUBLIC p2p_network
  PUBLIC Boost::system
  PUBLIC bitcoin_utils
  PUBLIC easylogging
  PUBLIC miniupnpc-static
  -static-libgcc -static-libstdc++)

if(NOT MSVC)
  target_compile_options(${PROJECT_NAME} PRIVATE -Wall -Wextra -Wno-unknown-pragmas -pedantic)
else()
  target_compile_options(${PROJECT_NAME} PRIVATE /W4)
endif()
