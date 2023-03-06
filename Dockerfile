FROM rust:1.67

RUN apt-get update
RUN apt-get install -y alsa-tools librust-alsa-sys-dev libudev-dev libopus-dev

WORKDIR /usr/src/cringe_radio

COPY . .

RUN cargo install --path .

CMD ["cringe_radio"]
