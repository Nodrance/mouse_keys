# Mouse Keys

This program allows you to use your keyboard to control your mouse.

## Controls
- ` (or ~): Enable/disable (toggle)
- Space: Left click (hold)
- Enter: Right click (hold)
- \  (or |): Middle click (hold)
- < > (or , .): Scroll up/down (scroll left/right exists in the code but has no button set for it)
- Tab: Change mode (this controls how the mouse speeds up as you hold the button)
- \+ and - (or _ and =): Change the speed of the current mode (applies next time you press a direction)
- LCtrl: big step. move 4x as far, and change speed 4x as much
- LAlt: small step. move 1/4 as far, and change speed 4x slower
- Backspace: reset speeds and go back to mode 1
- Numrow 0-9: Load slot 0-9
- Lshift + Numrow 0-9: Save slot 0-9

## Settings
Settings.rs has all the default settings, including keybinds, speeds, and more. It also has some comments on how to change them and what each one does. Settings.yaml does not work yet

## Installation
1. Download the code as a zip file using the green "Code" button on github, then "download as zip"
2. Unzip it somewhere (this example assumes it will be into a folder called "mouse_keys" inside your downloads folder)
3. Install cargo
4. In the windows search box, type in "command prompt" and open it
5. Type `cd Downloads/mouse_keys`, then type `cargo build --release`
6. In your file browser, go to target, then release, then run mouse_keys.exe

### Alternate method
1. Install git
2. Install cargo
3. In the windows search box, type in "command prompt" and open it
4. Type `cd Downloads` (or any other folder you like)
5. Type `git clone https://github.com/Nodrance/mouse_keys.git`
6. Type `cd mouse_keys`
7. Type `cargo build --release`
8. In your file browser, go to target, then release, then run mouse_keys.exe
To update, just type `git pull` to get the latest code changes