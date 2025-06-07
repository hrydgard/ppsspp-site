# PPSSPP on WSL

WSL or Windows Services for Linux is a convenient way to run a Linux VM on Windows. It's well-integrated and quite convenient.

Recently, it has even added graphics support and it's now possible to build and run PPSSPP on WSL2, which can be very useful for debugging - especially for running various tools that only exist or work better on Linux like `valgrind`, address-sanitizer etc.

## Tips

* Install VSCode on Windows (not in your WSL VM!)

* Check out the PPSSPP repository:

  ```sh
  git clone git@github.com:hrydgard/ppsspp.git --recursive
  ```

* Install the usual dependencies documented [here](https://github.com/hrydgard/ppsspp/wiki/Build-instructions#building-with-cmake-other-platforms-eg-linux) (plus cmake).

* Install at least the following VSCode extensions:

  - CodeLLDB (a debugger)
  - WSL
  - C/C++ Extension Pack

In your WSL directory where you checked out the PPSSPP git repo, from terminal, just:

```sh
code .
```

PPSSPP now provides `.vscode/tasks.json` and `.vscode/launch.json` for your convenience, so debugging should "just work".

The build task just runs a debug build, important to be aware of. Change it to `--release` or add an additional task if desired.

## Valgrind

Here's an example where we both use suppressions, and generate new ones (that you can then take from `suppressions.log` and simplify and copy to `valgrind-wsl2.supp`):

```sh
valgrind --suppressions=SDL/valgrind-wsl2.supp --gen-suppressions=all --log-file=suppressions.log build/PPSSPPSDL
```