> [!CAUTION]
> this softwere is currently under devolopment and does not fully work, *yet*

# Anima
a cli app that tracks you anime progress and wishlists **locally**, it uses modifyed data from [manami-project/anime-offline-database](https://github.com/manami-project/anime-offline-database)

## Download
nightly builds for [windows](https://nightly.link/yusuf-suterwala/anima/workflows/build/main/windows-debug-build.zip) and [linux](https://nightly.link/yusuf-suterwala/anima/workflows/build/main/linux-debug-build.zip)

## Setup
set the `ANIDB_FOLDER` environment variable to any folder you want the anime list and database to be stored

## Usage
### for creating the local anime bd

```anima create```

### for updating the user list
```
anima update-list {mal id} --status {WATCHING|COMPLATED|PAUSED|DROPED|PLANNING} \
                           --ep {int} --score {float} \
                           --started-ts {unix timestamp} --complated-ts {unix timestamp}
```
