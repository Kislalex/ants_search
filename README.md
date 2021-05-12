# AntsSearch

Implementation of an eurystic search of a shortest path using the ants algorythm.

### Prerequisites

This program runs on Rust. Requires installed rust and cargo.

Download the .zip archiv or clone the repo

```
git clone ...
```

And run the
```
cargo run
```
in the directory ant_search
Afterwards you can find binary executable file in "/ants_search/target/degub/"

### SDL for Windows (MinGW)
On Windows, make certain you are using the MinGW version of SDL; the native
version will crash on `sdl2::init`.

1. Download mingw development libraries from
http://www.libsdl.org/ (SDL2-devel-2.0.x-mingw.tar.gz).
2. Unpack to a folder of your choosing (You can delete it afterwards).
3. Copy all lib files from
    > SDL2-devel-2.0.x-mingw\SDL2-2.0.x\x86_64-w64-mingw32\lib

    to (for Rust 1.6 and above) 
    > C:\Program Files\Rust\\**lib**\rustlib\x86_64-pc-windows-gnu\lib

    or to (for Rust versions 1.5 and below)
    > C:\Program Files\Rust\\**bin**\rustlib\x86_64-pc-windows-gnu\lib
    
    or to your library folder of choice, and ensure you have a system environment variable of
    > LIBRARY_PATH = C:\your\rust\library\folder

	For Multirust Users, this folder will be in
	> C:\Users\{Your Username}\AppData\Local\.multirust\toolchains\{current toolchain}\lib\rustlib\x86_64-pc-windows-gnu\lib

4. Copy SDL2.dll from
    > SDL2-devel-2.0.x-mingw\SDL2-2.0.x\x86_64-w64-mingw32\bin

    into your cargo project, right next to your Cargo.toml.
