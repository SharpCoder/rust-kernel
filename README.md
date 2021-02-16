# armv7a-rust-os
This project is designed to run a baremetal rust program on beaglebone black. 

## Compiling
Simply run the `build.sh` script. This will generate `out/` files. The final is `rom.img` which you can flash to an SDCard. From there, you must hold down the "user program" button (labeled S2 on my beaglebone black) in order for the image to load.

## Resources
Documentation
http://www.sal.wisc.edu/st5000/datasheets/tq-systems/spruh73p.pdf

## License
[MIT](https://tldrlegal.com/license/mit-license)