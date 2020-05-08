This project is a button sending command when it's pressed.

Currently used with https://github.com/Vrixyz/music.

# Schematics
- Bluetooth connection is used via a HC-05 module, connected as directed by https://rust-embedded.github.io/discovery/13-serial-over-bluetooth/index.html.
- This trick might be worth reading if your board has a solder bridge missing like mine: https://rust-embedded.github.io/discovery/06-hello-world/index.html
- External button is wired with a pull down resistor like this: ![button schematics](doc/button_schematics.png)