# tomata

[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fkhrynczenko%2Ftomata%2Fbadge%3Fref%3Dmaster&style=flat)](https://actions-badge.atrox.dev/khrynczenko/tomata/goto?ref=master)
![GitHub](https://img.shields.io/github/license/khrynczenko/tomata)
![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/khrynczenko/tomata)

## What is tomata

**Tomata** is a cross-platform
[Pomodoro](https://en.wikipedia.org/wiki/Pomodoro_Technique)
(micro time-management) application for desktops written in *rust*.
The goal of *tomata* is to provide a
Pomodoro application that is easily adaptable to the requirements
of the user, i.e., provides wide set of settings.

### Systems

Although *tomata* should be working on macOS it was only sufficiently
tested on Linux(Ubuntu) and Windows only.

## Features

- Typical Pomodoro functionality (stopwatch, three different intervals)
- Adjustable duration of each period
- Adjustable number of short breaks
- Optional long breaks
- Optional system notifications on changing period
- Optional sound effect when period is ending

## How to build

**`cargo build` :)**

Unfortunately, it might be the case that on Linux some additional
packages will be required like `libgtk-3-dev`, `libasound2-dev`, and
`libdbus-1-dev`. This list might not be complete. To check what is needed
on a fresh system I encourage you to check CI scripts. You can find them
in [.github/workflows/](https://github.com/khrynczenko/tomata/blob/master/.github/workflows/).

## Obligatory screenshot

![tomata-screenshot](/screens/screen1.png)
