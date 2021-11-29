ifeq ($(shell uname),Darwin)
    LDFLAGS := -Wl,-dead_strip,-framework,Security
    #LDFLAGS := -Wl,-dead_strip
else
    LDFLAGS := -Wl,--gc-sections -lpthread -ldl
endif

all: target/hb
	target/hb

target:
	mkdir -p $@

target/hb: target/main.o ../../target/debug/libhb_ffi.a
	$(CC) -o $@ $^ $(LDFLAGS) -v

../../target/debug/libhb_ffi.a: src/lib.rs Cargo.toml
	cargo build

target/main.o: csrc/main.c | target
	$(CC) -o $@ -c $<

clean:
	rm -rf target
