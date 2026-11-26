# Sprout
sprout is a WIP build system that is meant to eventually be a more minimal alternative to the Ninja build system.
It is not yet complete, has hardcoded values, and only generates object files.

## Testing
```bash
git clone https://github.com/voltiksrc/sprout
cd sprout
cargo run
```

### Warnings
It does not accept CLI arguments, you must supply the hardcoded values which are located in the source,
it is very unfinished, make a better testing `main.c` file as you wish.

### Configuration/PLANT
PLANT is the eventual configuration file which describes what targets and such you want to build,
the syntax is only a demonstration! it does not yet affect anything but I will work on it soon.

#### Notes for users
`src/bin/dfs.rs` is the full basic dfs resolver file, the dependency resolution is not yet integrated into sprout,
try sprout if you want, it is very unfinished.

### LICENSING
sprout is licensed under the MIT license.
look at the [LICENSE](LICENSE) file for more details.
