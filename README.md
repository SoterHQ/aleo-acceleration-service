# Aleo Wallet & Acceleration Service

## [Soter Discord](https://discord.gg/Z9EhXYvh)

## How to install

### 1. Install Soter Wallet

- [Download](https://chromewebstore.google.com/detail/soter-aleo-wallet/gkodhkbmiflnmkipcmlhhgadebbeijhh)

### 2.Install aleo-acceleration-service

- [Download](https://github.com/SoterHQ/aleo-acceleration-service/releases/latest) the latest released version of your platform. macos(\*.dmg) windows(\*.msi)

#### Mac

- you need to enable application from any source, drag the app to application folder, then run the following command:

```bash
sudo xattr -r -d com.apple.quarantine /Applications/aleo-acc-service.app
```

#### Linux

1. Install

Install aleo-acc-service_x.x.x_amd64.AppImage Or aleo-acc-service_x.x.x_amd64.deb

2. Install from source code

```shell
./build-linux.sh
```

#### Windows

Install aleo-acc-service_0.0.x_x64_en-US.msi

### 3. Configure the ACC service

- Copy `server url` from ACC service, for example: `http://36604057fe67563e8e162f935a5c2fe1576adf4be5d8a322c5fd39e25c675ebb@127.0.0.1:18340`
<img width="793" alt="image" src="https://github.com/SoterHQ/aleo-acceleration-service/assets/148941726/3123ca79-6ea9-4551-b6a6-3c40ddba3432">

- Input the service address on chrome extension:
<img width="606" alt="image" src="https://github.com/SoterHQ/aleo-acceleration-service/assets/148941726/81b5c1a8-f75f-4f2f-8023-700edbc15567">


### 4. Now, when the method is executed, the local service will be called by default

### 5. Test Result "Send"

<img width="596" alt="image" src="assets/c1b255e4-1a65-44b6-bf70-8d680207176b.png">
