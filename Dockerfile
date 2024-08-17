FROM ubuntu:latest


RUN apt update 
# Install curl
RUN apt install -y curl
# Install templify
RUN curl -s https://raw.githubusercontent.com/cophilot/templify/master/install | bash -s -- -y
