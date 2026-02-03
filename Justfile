set dotenv-load := true

DEFAULT_EXTENSION := `echo ${DEFAULT_EXTENSION:-"y4m"}`
DEFAULT_LOCAL_DIR := `echo ${DEFAULT_LOCAL_DIR:-"$HOME/video"}`
DEFAULT_INSTALL_NAME := `echo ${DEFAULT_INSTALL_NAME:-"$(pwd | xargs -n 1 basename)-$(git branch --show-current)"}`
DEFAULT_INSTALL_DIR := `echo ${DEFAULT_INSTALL_DIR:-"$HOME/.local/bin"}`

all: clean release install
clean:
	cargo clean
build:
	cargo build
release:
	cargo build --release
install INSTALL_DIR=DEFAULT_INSTALL_DIR INSTALL_NAME=DEFAULT_INSTALL_NAME: build
	cp target/debug/dav1d {{INSTALL_DIR}}/{{INSTALL_NAME}}
install-release INSTALL_DIR=DEFAULT_INSTALL_DIR INSTALL_NAME=DEFAULT_INSTALL_NAME: release
	cp target/release/dav1d {{INSTALL_DIR}}/{{INSTALL_NAME}}
check VIDEO EXTENSION=DEFAULT_EXTENSION DIRECTORY=DEFAULT_LOCAL_DIR: release (test VIDEO)
	ffprobe -v error -select_streams v:0 -show_entries stream=y4m -of default=nokey=1:noprint_wrappers=1 {{DIRECTORY}}/{{VIDEO}}-decode.{{EXTENSION}}
test VIDEO EXTENSION=DEFAULT_EXTENSION DIRECTORY=DEFAULT_LOCAL_DIR: release
	./target/release/dav1d --threads 1 -i {{DIRECTORY}}/{{VIDEO}}.ivf -o {{DIRECTORY}}/{{VIDEO}}-decode.{{EXTENSION}}
cleanup VIDEO EXTENSION=DEFAULT_EXTENSION DIRECTORY=DEFAULT_LOCAL_DIR:
	rm {{DIRECTORY}}/{{VIDEO}}-decode.{{EXTENSION}}
convert VIDEO EXTENSION=DEFAULT_EXTENSION DIRECTORY=DEFAULT_LOCAL_DIR: release (test VIDEO) && (cleanup VIDEO)
	ffmpeg -i {{DIRECTORY}}/{{VIDEO}}-decode.{{EXTENSION}} {{DIRECTORY}}/{{VIDEO}}.mp4
