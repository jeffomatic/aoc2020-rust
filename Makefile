folders = $(shell find . -maxdepth 1 -type d -name "day*" | sed 's|^\./||')

.PHONY: no_default $(folders)
no_default:

$(folders):
	@ cargo build --manifest-path $@/Cargo.toml
	@ if [ -f $@/input ]; then cat $@/input | RUST_BACKTRACE=1 $@/target/debug/$@; else RUST_BACKTRACE=1 $@/target/debug/$@; fi
