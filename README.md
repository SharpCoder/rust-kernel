# rust-kernel
Welcome! This project is designed to run a baremetal rust program on the [beaglebone black](https://beagleboard.org/black). I am actively developing the various modules for this project, and you can expect more support for board features over time. 

## Technical Articles
My motivation for this project is primarily to act as a forcing function to help me practice writing technical articles. Each module I develop will eventually be turned into a blog post. Here is a list of the documentation I've written so far:

 - Compiling: https://sharpcoder.medium.com/baremetal-rust-for-beaglebone-black-part-1-compilation-6609cdc2545c
 - Creating a Stack Datastructure: https://medium.com/baremetal-rust/baremetal-rust-for-beaglebone-black-implementing-a-stack-6da5af43f677 


## Compiling

### Configuring your environment
You will need the following in order for the `build.sh` script to run successfully.
```
# Install gcc tools
sudo apt-get install gcc-arm-none-eabi

# Configure rust
rustup default nightly
rustup target add arm-unknown-linux-gnueabihf
```

### Building

Simply run the `build.sh` script. This will generate `out/` files. The final is `rom.img` which you can flash to an SDCard. From there, you must hold down the "user program" button (labeled S2 on my beaglebone black) in order for the image to load.


### Flashing the rom image

The `rom.img` which is generated as part of the build process is a valid `.img` file and can be flashed with a variety of tools. On Windows, I use the **Win32 Disk Imager** application.

## Technologies
This project is primarily written in [rust](https://www.rust-lang.org).


## Resources

 - (Primary board documentation) [http://www.sal.wisc.edu/st5000/datasheets/tq-systems/spruh73p.pdf](http://www.sal.wisc.edu/st5000/datasheets/tq-systems/spruh73p.pdf)
 - (My articles) [https://medium.com/baremetal-rust](https://medium.com/baremetal-rust)
 - (Details about Beaglebone) [https://beagleboard.org/black](https://beagleboard.org/black)


## License
[MIT](https://tldrlegal.com/license/mit-license)