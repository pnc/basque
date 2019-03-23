FROM debian:unstable-slim
COPY ./prepare-valgrind.sh ./prepare-valgrind.sh
RUN ./prepare-valgrind.sh
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
COPY ./build.rs ./build.rs
COPY ./wrapper.h ./wrapper.h
RUN touch basque.rs
ENV PATH="${PATH}:/root/.cargo/bin/"
RUN cargo build
COPY . /app
ENV PATH="${PATH}:/root/.cargo/bin/"
RUN cargo build
