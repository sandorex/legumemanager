# legumemanager
**Superceeded by [arcam](https://github.com/sandorex/arcam)**

WIP container manager written in rust, mainly intended for rootless containers but with more emphasis on automation with planned ansible support

## Compared to distrobox
Distrobox is opiniated, but at the same time does too much and too litle

- It will download and install your shell inside the container but at the same time forces you to set hostname manually using hacky init scripts
- Each container has too much predefined at creation, instead of immitating a VM and setting most of things at runtime
- It is written in rust compared to /bin/sh so should be easy to make it portable static binary
- Everything is inside the same binary

