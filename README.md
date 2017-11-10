# compromise
An image compression fixing tool for Twitter users.

By default Twitter compresses PNG images to save space.
This can be circumvented by making 1 pixel slightly transparent,
in which case the server just stores it uncompressed. This tool
automates this process making it easy to upload images without losing quality.

## Usage

```
compromise 1.0
videah <videah@selfish.systems>

twitter image compression fixing tool

USAGE:
    compromise [FLAGS] [OPTIONS] --input <INPUT>

FLAGS:
    -h, --help       Prints help information
    -s, --silent     run (and fail) silently
    -V, --version    Prints version information

OPTIONS:
    -i, --input <INPUT>      path to image to be fixed
    -o, --output <OUTPUT>    path to save resulting fixed file
```

## Results
At the time of writing (10th November, 2017) the tool gives the following boost to image quality.
These are from the same picture uploaded twice, one before using `compromise`, and one after using it.
![](before-after.png?raw=true)

## License

`compromise` is licensed under the MIT license. Please read the [LICENSE](LICENSE) file in this repository for more information.

## Mentions
Go check out [ravenworks](https://github.com/omgitsraven)'s [tool](http://ravenworks.ca/twitimagefix/) which is similar to `compromise` but in web app form

## TODO
- [ ] GUI Version
- [ ] Improve silence handling
- [ ] Allow multiple images to be fixed at once
