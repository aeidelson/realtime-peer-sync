Defines the network API between peers. This package shouldn't be exposed outside of this directory,
as it's not intended for outside use.

Protobuf source files are located in the `protos` directory. Calling `./regenerate_protos.sh` will
create new rust bindings in the `gen` directory.
