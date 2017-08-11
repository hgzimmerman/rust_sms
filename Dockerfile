FROM greyltc/archlinux

WORKDIR /app

#ADD . /app

RUN source /root/.bashrc; source /root/.bash_profile
RUN pacman -Syu --noconfirm
RUN pacman -S base --noconfirm
RUN pacman -S systemd --noconfirm
RUN pacman -S vim --noconfirm
RUN pacman -S make --noconfirm
RUN pacman -S gcc --noconfirm
RUN pacman -S openssl --noconfirm
RUN pacman -S postgresql --noconfirm
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
ENV PATH "/root/.cargo/bin:$PATH"
RUN cargo install diesel_cli --no-default-features --features postgres


ENV OPENSSL_DIR=/app/openssl

EXPOSE 8080
