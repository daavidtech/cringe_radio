FROM rust:1.67

RUN apt-get update
RUN apt-get install -y alsa-tools librust-alsa-sys-dev libudev-dev libopus-dev ffmpeg

RUN wget -O /usr/bin/youtube-dl https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux
RUN chmod +x /usr/bin/youtube-dl

WORKDIR /usr/src/cringe_radio

COPY . .

RUN cargo build --release

CMD ["./target/release/cringe_radio"]
