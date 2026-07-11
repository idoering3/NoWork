# NoWork

NoWork is a passion project designed to keep track of tasks, focused on providing tools to drive focus and study, as well as task management. I wanted to install a task app, but found that I was not happy with the vast majority of them. NoWork intends to fix that, with version 1.1 aimed at providing the basic features needed to daily life, and version 1.2 aimed at delivering the app to devices that are not desktop computers such as mobile.

<img width="1912" height="1024" alt="image" src="https://github.com/user-attachments/assets/1de67bf9-9034-498e-802c-c691efe074e1" />

# Releases
NoWork officially released in 1.0 in December of 2025. However, due to various issues, the original repository had to be deleted and re-created. NoWork is quickly approaching version 1.1, which includes a reworked home bar, Apple calendar support, and more advanced tasks.

## Roadmap

NoWork 1.2 will be released by the end of 2026. It is intended to allow for self-hosting of a NoWork server, allowing users to utilize Tailscale from any device to access their tasks. Repetitive tasks will also be implemented in this release. The study tab will also be revamped in this version.

# Installation

## Windows
There will be builds for the windows version of NoWork, in both .msi and .exe formats.

## Linux
There are currently no builds for the linux version of NoWork.

# Building on Other Systems
For builds in other versions, download the project files and build the project from scratch. For more information, see [Building in Tauri](https://tauri.app/distribute/)

# Making Custom Versions
Npm and tauri are required to run the dev version. Run ```npm install```. Then run ```npm run tauri dev```.
To build, simply run ```npm run tauri build```.
