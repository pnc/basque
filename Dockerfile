FROM debian:unstable-slim
RUN echo "deb http://deb.debian.org/debian-debug/ unstable-debug main" > /etc/apt/sources.list.d/source.list
RUN apt-get update && apt-get install -y clang curl sqlite3-dbgsym sqlite3 libsqlite3-dev valgrind
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
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
