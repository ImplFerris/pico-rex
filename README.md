# pico-rex
Dinosaur Game written in Rust for the Pico 2 (RP2350) with an OLED display, using the Embassy framework.

## Hardware Requirements
- Pico 2 (RP2350)
- SSD1306 OLED I2C 128x64 Display
- Push button (with a cap) 
- Jumper wires and breadboard

## Circuit

| Pico Pin | Component               |
|----------|-------------------------|
| GPIO 18  | SDA pin of OLED         |
| GPIO 19  | SCL pin of OLED         |
| 3.3V     | VCC pin of OLED         |
| GND      | GND pin of OLED         |
| GPIO 15  | One side of push button |
| GND      | Other side of push button |


## TODO
1. Implement running illusion for the T-Rex
2. Display start menu
3. Smooth gaming experience!



