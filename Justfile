# TODO: Currently, you can only use Release, this is bad, fix
DEFAULT_DIRECTORY := `echo ${DEFAULT_DIRECTORY:-"$HOME/video"}`
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
check VIDEO DIRECTORY=DEFAULT_DIRECTORY: release (test VIDEO)
	ffprobe -v error -select_streams v:0 -show_entries stream=y4m -of default=nokey=1:noprint_wrappers=1 {{DIRECTORY}}/{{VIDEO}}-decode.y4m
test VIDEO DIRECTORY=DEFAULT_DIRECTORY: release
	./target/release/dav1d -i {{DIRECTORY}}/{{VIDEO}}.ivf -o {{DIRECTORY}}/{{VIDEO}}-decode.y4m --framedelay 1 --threads 1
test-convert VIDEO DIRECTORY=DEFAULT_DIRECTORY: (test VIDEO DIRECTORY) (convert VIDEO DIRECTORY) && (cleanup VIDEO DIRECTORY)
cleanup VIDEO DIRECTORY=DEFAULT_DIRECTORY:
	rm {{DIRECTORY}}/{{VIDEO}}-decode.y4m
convert VIDEO DIRECTORY=DEFAULT_DIRECTORY:
	ffmpeg -i {{DIRECTORY}}/{{VIDEO}}-decode.y4m {{DIRECTORY}}/{{VIDEO}}.mp4
