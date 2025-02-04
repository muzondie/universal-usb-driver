# Universal USB Driver  

A tool to automatically install drivers for USB devices on Windows. It detects connected hardware, finds the correct drivers, and handles setup without user input. Works with all major brands and device types (keyboards, storage, controllers, etc).  

## Download  
1. Go to the [Releases](https://github.com/muzondie/universal-usb-driver/releases) tab.  
2. Download the latest `.zip` file.  
3. Unzip the file.  
4. Run `UniversalUsbDriver.exe`.  

## Usage  
1. **Run** the application.  
2. **Plug in** your USB device.  
3. Wait for automatic detection and installation (5-30 seconds).  
4. The driver runs in the background after setup.  

## Features  
- Works on 64-bit Windows 10, 11, and newer.  
- Detects printers, webcams, gamepads, external drives, and other USB devices.  
- No manual driver searches or downloads.  
- Supports devices from Dell, Logitech, Samsung, Sony, HP, and others.  
- Lightweight (under 15 MB).  
- Offline driver database included, no internet required after installation.  
- Updates drivers for existing devices.  
- Silent installation mode for enterprise use.  
- Uninstall tool removes all drivers added by the app.  

## Build from Source  
1. Install [Rust](https://www.rust-lang.org/tools/install).  
2. Open a terminal and run:  
   ```  
   git clone https://github.com/muzondie/universal-usb-driver  
   cd universal-usb-driver  
   cargo build --release  
   ```  
3. Find the executable in `target/release/`.  

## Contributing  
Code contributions and feature requests are currently paused due to limited maintenance capacity.  

## License  
MIT [License](LICENSE).