# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

## [0.24.5.26]

### Fixed
- Bugfix in print-completion loop for shells other than zsh (Andrew Davis)
- Fixed a problem in GitHub Actions that ran all the tests on the same python version instead of all the python versions specified
- Fixed regression that would crash trash-restore on non parseable trashinfo, with an error like: "TypeError: not enough arguments for format string"

### Changed
- Move all the main logic for dev tools from scripts under tests/support to solve issue https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=1067303
- Reintroducing testing on python 2.7 thanks the custom GitHub action found at https://github.com/ytdl-org/setup-python
- Using python -m venv instead of virtualenv
- Refactor of some code in trash-put and trash-restore

## [0.24.4.17]

### Added
- Add `fuse.gocryptfs` to the mount_points to the list of allowed filesystems (Maxim Baz)
- Improved shell TAB-completion (Giuseppe Stelluto)

### Fixed
- Fix a regression that would crash `trash-list --volumes` at every launch
- Fix a bug that would trash a link destination instead of trashing the link when the file to be trashed contains trailing slashes
- Fix typos/grammar in man pages (qadzek)

### Changed
- trash-put --help shows also other commands (Joseph Masone)
- trash-restore prints a message when no file are restored: "No files were restored" (lukasvrenner)
- Made GitHub Actions workflows to work again
- Updated the actions versions in GitHub Actions
- Made the tests work again against python 2.7
- Fixed the tox configuration (in some cases it would run python 2.7 tests on python 3!)
- Made the type checker happy again
- The tarball created by python -m build sdist changed name, a dash "-" became a underscore "_"

## [0.23.11.10]

### Changed
- Use enum34
- MyPy improvements
- trash-put clearer error messages

## [0.23.9.23]

### Added
- Man page trash-put.1 will also document trash-dirs locations (Joel Pereira)
- Add MyPy checks to the GitHub Action test workflow
- Suggested to use pipx in README.md (bryango)
- Add instruction for installation on Fedora (Mohammed Eshan)

### Fixed
- Fixed typos in man page trash-put.1 (Joel Pereira)
- Improved error message when TrashInfo is not parsable in trash-restore
- Fixed typos (David Auer)
- Check info target trash folder non-exists (Tin Lai)

### Changed
- Removed python 2.7 from the list of tested platforms (as GitHub Actions no longer support this Python version)
- Get back `--` the command ending for trash-put (@laggardkernel)

## [0.22.10.20]

### Added
- Made visible option -v/--verbose of trash-empty (Andrea Francia)
- Documented how completion works in README (Wu Zhenyu)
- Add tox

### Changed
- Now if a file does not exist it does not try to delete it using all available trash dirs (Andrea Francia)

### Fixed
- Removed an error that would occur on trash-put when HOME environment is not set (Andrea Francia)
- Whitelisted fuse.glusterfs filesystem (Andrea Francia)

## [0.22.10.4.4]

### Added
- Add shell completions by shtab
- Feature: trash-empty learnt the --verbose option
- Feature: trash-empty learnt the --dry-run option
- Add more debugging messages to trash-put when it fails to trash a file
- Add `trash-list --python` to print the python executable

### Changed
- Revisited the trash-put log messages
- Add six as a requirement
- Add 'fuse' to the list of "physical" file system types
- Now trash-put uses ArgumentParser instead of OptionParser
- trash-empty now uses lexists (instead of exists) to check if a file is not existent before removal

## [0.22.8.27]

### Added
- `trash-list --all-users` to see trash from all users

### Changed
- Partitions with fs in ['nfs4','btrfs'] are considered physical volumes
- Partitions mounted on /tmp with tmpfs will be considered physical volumes

## [0.22.8.21.16]

### Added
- Now supports p9 (WSL 2 volumes) as location for trash dirs
- trash-list --volumes to list all the recognized volumes
- trash-list --debug-volumes

### Fixed
- Fix links to trash specification (David Auer)

## [0.22.8.21]

### Fixed
- Fix a bug that made `trash-list --size` crash if it found a broken link in the trash directory files
- trash-empty do not list trash directories that do not exist
- Fix trash-empty not showing nfs mountpoints
- Fix perms for user's trash folder

## [0.22.4.16]

### Fixed
- trash-restore exits gracefully if the user enters Ctrl+D

## [0.21.10.24]

### Added
- trash-empty learnt the -i/--interactive option

### Changed
- trash-empty detect when input is interactive and asks before emptying trash
- trash-empty option --all-users is no longer hidden

## [0.21.7.24]

### Fixed
- Fix bug in tests

## [0.21.7.23]

### Fixed
- Fix bug in tests (see https://github.com/andreafrancia/trash-cli/issues/210)

## [0.21.6.30]

### Added
- Now `trash-empty` honors multiple --trash-dir options
- trash-empty learnt the hidden option --print-time and now uses TRASH_DATE environment variable if present
- trash-empty learnt the --all-users option

### Changed
- `trash-empty --help` now shows only the command basename and not the full path to the command

## [0.21.5.25]

### Added
- Now trash-put honors the -i option (also the --interactive one)

## [0.21.5.22]

### Fixed
- trash-rm: fixed pattern matching for absolute paths, fixes https://github.com/andreafrancia/trash-cli/issues/124

## [0.21.5.20]

### Added
- Add (hidden and undocumented) --files option to trash-list

## [0.21.5.11]

### Added
- trash-put also accept -vv for enabling debug prints
- Add (hidden and undocumented) --size option to trash-list

## [0.21.4.18]

### Added
- trash-list learnt the `--trash-dir` option
- trash-list shows all partitions including not physical (@KSR-Yasuda)

### Fixed
- Fix bug #166 'trash goes into an infinite loop when trashing files with a long filename'
- trash-restore now supports relative paths in argument (fixes #165)

## [0.20.12.26]

### Added
- trash-restore learnt --trash-dir option
- Add simplified Chinese README
- Add to README the installation with apt
- trash-restore now supports range select

### Changed
- trash-restore now uses 'date' as the default sort argument

## [0.20.11.23]

### Changed
- Switched to psutil for listing volumes

## [0.20.11.7]

### Added
- trash-put learned a --trash-dir option that can be used to specify the trash directory to be used as destination
- trash-restore learnt --sort=(date|path|none) option (Self-Perfection)
- trash-restore: support restoring multiple files (arendu)

### Changed
- trash-put -f now ignores files and dirs that do not exist (Don Cross)
- README: now recommend using `pip` for installing trash-cli

## [0.17.1.14]

### Fixed
- Fix a bug that causes trash-put to use $topdir/.Trash/UID trashcan even when it is not secure and $topdir/.Trash-UID should be used instead

## [0.17.1.12]

### Fixed
- Fix a bug in detecting right volume of home trash dir, if home Trash dir is a symbolic link, and in detecting volume of file to be trashed when it is specified as contained in a directory that is a symbolic link that crosses volumes boundaries (#38)

### Changed
- Make some code python 3 compatible
- Fixed README

## [0.17.1.1]

### Added
- Now trash-rm supports full path matching, using a pattern starting with slash '/' (Fix #67)
- Add a reference to trash-rm(1) to all man pages
- Add support for --trash-dir option to trash-empty

### Fixed
- Fix typo in trash-rm(1) man page
- Fix inconsistent apostrophes

## [0.16.12.29]

### Fixed
- trash-rm no more crashes on .trashinfo files without Path (#69)

## [0.16.12.28]

### Fixed
- Fix #48 trash-empty crashes on directories without read permission

## [0.16.12.26]

### Fixed
- Fix #52 Almost all commands crash with python 2.7.10

## [0.16.12.25]

### Added
- Now trash-restore accepts a /specific/path
- Add input validation in trash-restore

### Changed
- Now integration tests should pass also in a linux box (Fix #61)
- Now all command outputs will report the right up-to-date location for issue reporting (#39)
- Renamed restore-trash to trash-restore
- Minor changes to man pages

### Fixed
- Fixed bug (trash-put creates $topdir/.Trash even if it should not)
- Fixed bug (trash-put uses $topdir/.Trash/$uid even if unsecure)

## [0.12.9.14]

### Added
- New trash-rm command

### Changed
- Switched to distutils.core (instead of setuptools)
- Now `trash-put -v` will warn if it found an unsticky .Trash dir
- (Internal) Switched from realpath to abspath

## [0.12.7]

### Fixed
- trash-empty crashed with GetoptError in short_has_arg(): option -2 not recognized
- Fixed inclusion of README.rst when creating distribution package

## [0.12.6]

### Added
- Add Donate button on README

## [0.12.4.24]

### Fixed
- Fixes a packaging problem of the previous release which prevented the installation via easy_install and/or pip
- Fixes the name of the man page for restore-trash

## [0.12.4]

### Added
- Reintroduced `trash` command as alias to `trash-put`

### Changed
- Now trash-list checks for $topdir/.Trash having sticky bit and not being a symlink and warns when these requirements are not met
- Now trash-list handles empty, unreadable or malformed .trashinfo
- Now `trash-empty <days>` skips .trashinfos with invalid DeletionDates
- Removed Unipath dependency
- Switched from googlecode to github
- Removed tests written in Bash
- Complete rewrite of trash-list and trash-empty

## [0.11.3]

### Added
- Now works also on Mac OS X

### Fixed
- Fixed #55: restore-trash sets all-write permissions for the destination directory
- Volumes detection: Now uses "df -P" output as fallback when getmnt fails
- Fixed #54. Now restore trash refuses to overwrite a file

## [0.11.2]

### Fixed
- Fixed #45: Cannot build RPM package with 0.11.1.2

## [0.11.1.2]

### Fixed
- Fixed problems running setup.py

## [0.11.1]

### Changed
- Updated version number to make easy_install happy

## [0.11.0]

### Fixed
- Fixed serious bug in trash-put: now the dot `.` and dot-dot `..` are skipped
