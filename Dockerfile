FROM rust:1.67

WORKDIR /usr/src/cringe_radio

COPY . .

RUN cargo install --path .

CMD ["cringe_radio"]
