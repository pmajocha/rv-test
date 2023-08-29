FROM rust:1.71-alpine3.18

RUN apk update \
    && apk upgrade \
    && apk add --no-cache \
    #autoconf \
    build-base \ 
    binutils \ 
    #cmake \ 
    #curl \ 
    #file \ 
    gcc \ 
    g++ \ 
    #git \ 
    libgcc \ 
    libtool \ 
    linux-headers \ 
    #make \ 
    musl-dev 
    #ninja \ 
    #tar \ 
    #unzip \ 
    #wget \
    #perl \
    #zip \
    #pkgconfig \
    #python3


WORKDIR /var/app

COPY ./built-libraries ./built-libraries
COPY ./src ./src
COPY ./re2/include ./re2/include
COPY build.rs .
COPY Cargo.toml .
COPY wrapper.cpp .

CMD ["cargo", "t"]