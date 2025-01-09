# Cropper

After Adobe turned their creative suite into an expensive subscription service, I was forced to stop using it. I just could not justify it with my sporadic and somewhat infrequent use. When I do run into common editing tasks now, I have to try to figure out how to do what I need using free tools. As a result, I sometimes throw together mini-tools for very common tasks. One common task is that I often take screenshots where I select a region with my trackpad. I then need to crop that image down to something I can use. This tool just makes that process a little easier.

## Usage

Create a preview that shows the region that would be cropped from the image. The cropped image is specificed by insets from the edges. `10x20-30x40` means 10 pixels from the left, 20 pixels from the top, 30 pixels from the right, and 40 pixels from the bottom.

```console
$ cropper preview --inset=10x20-30x40 input.png output.png
```

Crop the image.

```console
$ cropper crop --inset=10x20-30x40 input.png output.png
```
