all: clean release install
clean:
	cargo clean
build:
	cargo build
release:
	cargo build --release
install:
	cp target/debug/dav1d $HOME/.local/bin/dav1d
install-release:
	cp target/release/dav1d $HOME/.local/bin/dav1d
test-basic: release
	./target/release/dav1d -i $HOME/video/color.ivf -o $HOME/video/color-decode.y4m
	ffprobe -v error -select_streams v:0 -show_entries stream=y4m -of default=nokey=1:noprint_wrappers=1 $HOME/video/color-decode.y4m
	rm $HOME/video/color-decode.y4m
