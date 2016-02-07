# Change Log #
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

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
