cmake_minimum_required(VERSION 2.8.11)
project(qmlrswrapper)

set(CMAKE_BUILD_TYPE RELEASE)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_AUTOMOC ON)

include(CheckCXXCompilerFlag)
CHECK_CXX_COMPILER_FLAG("-std=c++11" COMPILER_SUPPORTS_CXX11)
CHECK_CXX_COMPILER_FLAG("-std=c++0x" COMPILER_SUPPORTS_CXX0X)
if(COMPILER_SUPPORTS_CXX11)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")
elseif(COMPILER_SUPPORTS_CXX0X)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++0x")
else()
    message(STATUS "The compiler ${CMAKE_CXX_COMPILER} has no C++11 support. Please use a different C++ compiler.")
endif()

find_package(Qt5Core)
find_package(Qt5Quick)

set(qmlrswrapper_SRCS libqmlrswrapper.cpp qrsdynamicobject.cpp)

add_library(qmlrswrapper STATIC ${qmlrswrapper_SRCS})
target_link_libraries(qmlrswrapper Qt5::Core Qt5::Quick)

install(TARGETS qmlrswrapper ARCHIVE DESTINATION lib)
