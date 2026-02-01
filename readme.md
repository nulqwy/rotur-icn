# rotur ICN renderer

a fast implementation of a renderer for ICN,
an icon format used within [originOS](https://origin.mistium.com/)

documentation about ICN can be found at [icn.rotur.dev](https://icn.rotur.dev/).

## usage

currently supports only exporting images in PAM format, so example usage may be:

```sh
$ icn-viewer export myicon.icn | magick - myicon.png
```

## features

- supports full ICN spec with full parity with orginal implementation
- invalid ICN detection and recovery
- canvas fitting, scaling, padding, panning
