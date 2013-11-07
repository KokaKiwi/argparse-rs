RUST_MODULES		:=	argparse
# RUSTCFLAGS			+=	--opt-level=3
# RUSTCFLAGS			+=	--opt-level=0 -Z debug-info -Z extra-debug-info

include				rust-mk/rust.mk

example:			$(RUST_BUILDDIR)/.build_argparse example.rs
	$(RUSTC) $(RUSTCFLAGS) -o example example.rs
all:				example

clean_example:
	@rm -f example

clean:				clean_example
