---
description: Trust installation documentation
---

# Installation

Trust installation is simple. installation is just a matter of decompressing a zip file and copying the binary to an appropriate directory; you can also use the package manager that came with your OS. here is a list of the ways you can install and run Trust

## Downloading a release build

* You can find the latest release of Trust on the [github releases page](https://github.com/zolamk/trust/releases).
* From the releases page, copy the link to the release archive file of your choice and download it

For example, assuming version `X.Y.Z` of the server and a `Linux AMD64`:

```
curl -L https://github.com/zolamk/trust/releases/download/vX.Y.Z/trust-vX.Y.Z-linux-amd64.tar.gz
```

then extract the archive

```
tar -xf trust-vX.Y.Z-linux-amd64.tar.gz --one-top-level
```

move/copy the extracted binary to a location of your choice

```
mv trust-vX.Y.Z-linux-amd64/trust /usr/bin/
```

finally run your binary

```
/usr/bin/trust run --config ./config.json
```
