# :knot: knot: prevent breaking file dependencies

`knot` lets you mark files and directories as dependant on others, so that operations like `mv`, `cp`, `rm` or `chmod` warn you before possibly breaking file dependencies.

## Example of use

To use `knot` quickly, it's advised to include it as an `alias`:

```sh
# file: .bashrc/.zshrc/...
alias mv='knot --run /usr/bin/mv'
```

```sh
$ knot project my_dependency
#=> [successfully created knot]
$ mv my_dependency my_new_cool_name
#=> knot> warning! `project` depends on `my_dependency`.
#=> knot> moving it might break functionality.
#=> knot> continue anyways? [yN]: y
#=> knot> ran successfully — please consider updating the dependency using `knot --change`.
$ knot --change my_dependency new_cool_name
#=> [successfully renamed dependency]
$ knot --remove project *
#=> [successfully removed all dependencies]
$ mv new_cool_name new_different_name
$
```

## Usage

TODO

## Project status

Vision.

## License

Licensed under the MIT license. Refer to [LICENSE](./LICENSE) for more detail.
