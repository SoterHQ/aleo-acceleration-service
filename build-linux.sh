git submodule update --init
sudo apt-get install -y librocksdb-dev \
    libssl3-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libc6-dev \
    libstdc++6-dev \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

yarn
yarn tauri build
yarn open
