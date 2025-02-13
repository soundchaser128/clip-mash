FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu-24.04

RUN wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2404/x86_64/cuda-keyring_1.1-1_all.deb && dpkg -i cuda-keyring_1.1-1_all.deb 
RUN apt-get update && apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev cuda-toolkit zlib1g cudnn9-cuda-12 cuda-drivers