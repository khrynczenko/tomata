# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- increased window height on Windows OS ("Save" button was not fully visible
  before)
- added link-time optimization to release builds
- denied more `rustc` lints

### Features

- added beep volume adjustment setting

## [0.1.0] - 2020-10-12

This is the initial release that brings the minimum set of required
features such as stopwatch, adjustable period durations, and many
settings that can be changed by the user. Initially, the application
was tested on Linux and Windows only. The continuos integration
was set using *github actions* in a very basic way.
