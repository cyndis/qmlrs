# Change Log #
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## 0.1.1 - 2016-03-09 ##
This release was brought to you thanks to @Kroisse and the excellent internet connection in the Tōkaidō Shinkansen line.
### Fixed ###
- fixed #39: increased required `pkg-config` version to `0.3.8` to include a fix for a MacOSX Compile Error

## 0.1.0 - 2016-02-07 ##
### Added ###
- Added a C++ wrapper around common QML-related features of Qt5, but not everything is already exposed to Rust code.
- Rust code can:
	- load QML code from files and strings.
	- execute QML code via QQmlApplicationEngine.
	- expose methods of Rust structs to QML code as context properties.
- Supported QVariant types that get converted automatically to Rust equivalents: Int64, Bool, String
- Now compiles cleanly on Linux, MacOS X, Windows & FreeBSD.


This uses [Keep a CHANGELOG](http://keepachangelog.com/) as a template.
