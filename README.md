# webprs
convert image to webp format inside a folder

## How to use?
For most situation, you can execute webprs with convert command directly. 
```bash
webprs convert
```
We also provide some optional arguments
```
// quality of webp images. range: 50~100. default value: 100
-q 80

// thread count that you want to create. default value: 5.
-t 5
```

## Install
You can download executable file for macOS from github releases.
Or you can clone this project and use `cargo` to compile executable file.

> For macOS user

> Unzip compressed file or compile this project, then put `webprs` into `/usr/local/bin`.
