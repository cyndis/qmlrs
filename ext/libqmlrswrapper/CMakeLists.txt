cmake_minimum_required(VERSION 2.8.11)
project(qmlrswrapper)

set(CMAKE_BUILD_TYPE RELEASE)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_AUTOMOC ON)

find_package(Qt5Core)
find_package(Qt5Quick)

set(qmlrswrapper_SRCS libqmlrswrapper.cpp)

add_library(qmlrswrapper STATIC ${qmlrswrapper_SRCS})
target_link_libraries(qmlrswrapper Qt5::Core Qt5::Quick)

install(TARGETS qmlrswrapper ARCHIVE DESTINATION lib)
