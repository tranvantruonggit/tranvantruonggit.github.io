# Builds the wasm core and drops it into the Jekyll assets (GitHub Pages can't run cargo).
TARGET  := wasm32-unknown-unknown
OUT     := target/$(TARGET)/release/hexcalc.wasm
DEST    := ../../assets/wasm/hexcalc.wasm

.PHONY: all clean

all: $(DEST)

$(DEST): src/lib.rs Cargo.toml
	cargo build --release --target $(TARGET)
	mkdir -p $(dir $(DEST))
	cp $(OUT) $(DEST)

clean:
	cargo clean
