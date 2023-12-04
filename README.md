# Aleo Wallet & Acceleration Service

## How to install

### 1.Install aleo-acceleration-service

- [Download](https://github.com/SoterHQ/aleo-acceleration-service/releases) the latest released version of your platform. macos(\*.dmg) windows(\*.msi)

#### Mac

- you need to enable application from any source, drag the app to application folder, then run the following command:

```bash
sudo xattr -r -d com.apple.quarantine /Applications/aleo-acc-service.app
```

#### Linux

1. Install

Install aleo-acc-service_x.x.x_amd64.AppImage Or aleo-acc-service_x.x.x_amd64.deb

2. Install from source code

Install dependent library
`sudo apt updatesudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`

```shell
git clone https://github.com/SoterHQ/aleo-acceleration-service.git
yarn
yarn tauri build
yarn tauri dev
```

#### Windows

Install aleo-acc-service_0.0.x_x64_en-US.msi

### 2. Install Soter Wallet

- [Download](https://chromewebstore.google.com/detail/soter-aleo-wallet/gkodhkbmiflnmkipcmlhhgadebbeijhh) 

### 3. Configure the ACC service

- Copy `server url` from ACC service, for example: `http://36604057fe67563e8e162f935a5c2fe1576adf4be5d8a322c5fd39e25c675ebb@127.0.0.1:18340`
<img width="800" alt="image" src="assets/Snipaste_2023-10-20_17-18-14.png">
- Input the service address on chrome extension:
<img width="718" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/018f78de-7522-4124-b6b2-fcd0616c8496">



### 4. Now, when the method is executed, the local service will be called by default

### 5. Test Result "Send"

  <img width="596" alt="image" src="assets/c1b255e4-1a65-44b6-bf70-8d680207176b.png">
