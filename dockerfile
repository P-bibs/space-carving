# syntax=docker/dockerfile:1
FROM ubuntu:18.04

ENV DEBIAN_FRONTEND=noninteractive
ENV LANG C.UTF-8
ENV PATH /usr/local/bin:$PATH

# Add apt-add-repository
RUN apt-get update && apt-get upgrade -y && apt-get install -y software-properties-common

# Install dependencies
RUN apt-get update && apt-get install -y \
    apt-utils libssl-dev lsb-release openssl vim git \
    build-essential libssl-dev zlib1g-dev libncurses5-dev \
    libncursesw5-dev libreadline-dev libgdbm-dev libdb5.3-dev libbz2-dev \
    libexpat1-dev liblzma-dev tk-dev libffi-dev wget unzip \
    curl libclang-dev python3-pip python-dev libjpeg8-dev 

# Hack to get python dependencies working
RUN ln -s /usr/lib/x86_64-linux-gnu/libjpeg.so /usr/lib

# install rust
RUN curl https://sh.rustup.rs | sh -s -- -y

# clear tmp folder
RUN rm -rf /tmp/*

# Do not copy, instead bind mount during docker run
RUN mkdir /home/space-carving

ENTRYPOINT "/bin/bash"

# Run with docker run -v .:/home/pelton