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
check VIDEO: release (test VIDEO)
	ffprobe -v error -select_streams v:0 -show_entries stream=y4m -of default=nokey=1:noprint_wrappers=1 $HOME/video/{{VIDEO}}-decode.y4m
test VIDEO: release
	./target/release/dav1d -i $HOME/video/{{VIDEO}}.ivf -o $HOME/video/{{VIDEO}}-decode.y4m --framedelay 1 --threads 1
test-convert VIDEO: (test VIDEO) (convert VIDEO) && (cleanup VIDEO)
cleanup VIDEO:
	rm $HOME/video/{{VIDEO}}-decode.y4m
convert VIDEO:
	ffmpeg -i $HOME/video/{{VIDEO}}-decode.y4m $HOME/video/{{VIDEO}}.mp4
