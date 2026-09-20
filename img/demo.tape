# SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
#
# SPDX-License-Identifier: GPL-3.0-or-later

Require any2nix

Set FontSize 22
Set Height 650
Set Theme "Github"
Set Width 1300

Type "any2nix -f toml rust-toolchain.toml"
Enter
Wait /\$/
Sleep 10s

Output demo.gif
