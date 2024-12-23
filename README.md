# pico-rex
Dinosaur Game written in Rust for the Raspberry Pi Pico 2 (RP2350) with an OLED display, using the Embassy framework.  You can also find the ESP32 version of this code [here](https://github.com/ImplFerris/esp32-rex).

## Hardware Requirements
- Raspberry Pi Pico 2 (RP2350)
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

## Example Video

The video doesn't fully capture the visual quality. I'm not great at recording videos, but here's a preview of the game.  

https://github.com/user-attachments/assets/7eb7a3f7-1ce7-49ca-9feb-2d63cbb8fcea



## TODO
1. Implement running illusion for the T-Rex
2. Display start menu
3. Smooth gaming experience!



