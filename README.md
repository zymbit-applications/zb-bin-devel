# zb-bin-devel


## Installing Zymbit tools
```
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/zymbit-applications/zb-bin-devel/main/install.sh | sh
```

### To install non-interactively:

- Download the installer from the "releases" section of this repo or build it yourself
- Run:
```
./zb-install [--with-hardware-signing | --with-software-signing] [--zb-version <latest|VERSION_TAG>]
```

**OR**

Add the `-s` option to `sh` and specify installer arguments:
```
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/zymbit-applications/zb-bin-devel/main/install.sh | sh -s -- <installer-args>
```
