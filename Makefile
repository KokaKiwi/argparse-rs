RUST_MODULES		:=	argparse
include				rust-mk/rust.mk

example:			$(RUST_BUILDDIR)/.build_argparse example.rs
	$(RUSTC) $(RUSTCFLAGS) -o example example.rs
all:				example

clean_example:
	@rm -f example

clean:				clean_example
