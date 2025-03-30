# Stereo Player Launcher

## Overview

Stereo Player Launcher provides an easy way to listen to music from the [Compute! Gazette Sid Collection](https://www.c64music.co.uk/), while being able to listen to stereo files, and see lyrics scroll and watch animations when present.

## Installation

1. Ensure that the [VICE emulator](https://vice-emu.sourceforge.io/) is installed. This launcher has been tested with VICE 3.9 and might not work with other versions.
1. Extract the files from the [latest release](releases/latest).
1. If `x64sc` is not on your PATH, update `stereo_player_launcher.ini` so that it points to the correct location.
1. Associate `stereo_player_launcher` with `.mus` files.

## Usage

1. Open a `.mus` file with `stereo_player_launcher`. The launcher will copy files with the same name and different extensions to a temporary directory, then run VICE.
1. Press F1 in VICE to start the song.

## Building

1. `cargo build --release`
1. Copy contents of `assets` directory and `stereo_player_launcher.ini` to same location as the executable.

## Notes

The stereoplayer105.d64 contains a modified version of [StereoPlayer 10.5](https://csdb.dk/release/?id=101655) with a skip of the intro notice, removal of fastloaders (VICE is configured instead), and configuration options set.

## Credits

Thanks to Mark A. Dickenson for StereoPlayer, Vanessa Ezekowitz for the upgrade to 10.5, and the VICE team for making an excellent Commodore 64 emulator.