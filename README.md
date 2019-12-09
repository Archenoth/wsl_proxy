# wsl_proxy: A way to use your WSL Linux binaries as Windows programs

Have you ever made an `ls.bat`? Or maybe even something that looked a little like this?

```batch
@echo off
bash -c 'git %*'
```

Thankfully, these batch files work pretty nicely if you don't want to install two versions of every tool you have; one for Linux, and one for Windows. But unfortunately, you can't set `.bat` files as the default program you open certain files with, and you can't readily use a lot of these with IDEs since they often simply don't accept `.bat` files in place of, for example, `git.exe`.

Enter wsl_proxy!

[![](https://asciinema.org/a/KMItxCIV03k7uXEyMRy6qjD6P.png)](https://asciinema.org/a/KMItxCIV03k7uXEyMRy6qjD6P)

If you rename it to `git.exe`, it will be WSL git. If you name it `emacs.exe`, it's gonna be WSL Emacs! You can put these in your path, give them to IDEs to use, and even assign your favorite Linux text editors as your default editor program, in Windows!

There is the added bonus that you don't need to configure multiple versions of things twice either! So if you have, for example, a fancy SSH configuration, not only can you still `ssh hostBehindProxyJumps` with your configs and your keys, your `git.exe` will also use them!

If you give wsl_proxy Windows paths, it will even translate them with `wslpath`, so you can run things like `git -C E:\SomePath\ log` and you'll get the result you probably intended. (IE: It Actually Working)

If you are space conscious of the binary size, but want a mirror of everything you have in Windows, you can even just hardlink everything you like in several WSL folders. (Yes, WSL hadlinking works on NTFS)

```bash
for bin in ls emacs vim nano git ssh; do
  ln wsl_proxy.exe $bin.exe
done
```

*Note:* `wsl_proxy.exe` needs to be somewhere Windows can see it. The binaries however, do not.
