# makectl    ![GitHub license](https://img.shields.io/github/license/Naereen/StrapDown.js.svg)](https://github.com/Naereen/StrapDown.js/blob/master/LICENSE)    ![made-with-bash](https://img.shields.io/badge/Made%20with-Bash-1f425f.svg)](https://www.gnu.org/software/bash/)    ![ForTheBadge built-with-love](http://ForTheBadge.com/images/badges/built-with-love.svg)](https://GitHub.com/Naereen/) 

Generate and Manage targets in your makefiles.

Makectl is a command line tool to generate and manage general use targets in your makefiles.

## Features

- Generates Makefiles and manages existing ones
- Provides a repository with templates for general use targets
- Does not break your custom targets

## Example

In a folder, lets say you have a `Makefile`

```make
.PHONY run

run:
    my_awesome_script --options

...
```

Now you may want to add some general use targets to reuse in your project, for example, everyone needs a target to clean up `.pyc` files in a Python project.

```bash
$ makectl add --template=python-clean
... Reading templates database from github.io/makectl...
... Building templates 
... Aplying new target `clean-pyc` to `./Makefile` 
```

The end result will be:

```make
.PHONY run clean-pyc

run:
    my_awesome_script --options

# MAKECTL MANAGED BLOCK INIT

clean-pyc:
	@find ./ -name '*.pyc' -exec rm -f {} \;
	@find ./ -name 'Thumbs.db' -exec rm -f {} \;
	@find ./ -name '*~' -exec rm -f {} \;
	rm -rf .cache
	rm -rf build
	rm -rf dist
	rm -rf *.egg-info
	rm -rf htmlcov
	rm -rf .tox/
	rm -rf docs/_build

# MAKECTL MANAGED BLOCK END
```

The templates database is a folder under this repo with `.template` files in it.
