import { p256 } from '@noble/curves/p256';
import { hkdf } from '@noble/hashes/hkdf';
import { sha256 } from '@noble/hashes/sha256';
import {
  DeployParams,
  ExecuteParams,
  TransferParams,
  JoinParams,
  SplitParams,
  JsonRpcResult,
  DiscoveryResult,
  JsonRpcRequest,
  DeploymentCostParams,
  ExecutionCostParams,
  DecryptRecordsParams,
  TransactionFromAuthorizationParams,
  DeployFromAuthorizationParams,
} from './types';
import { bytesToHex, hexToBytes } from '@noble/curves/abstract/utils';

export * from './types';
export class Client {
  privateKey: Uint8Array;
  publicKey: Uint8Array;
  serverurl: string;
  serverPubKey: Uint8Array;

  features: string[];

  private constructor(
    privateKey: Uint8Array,
    publicKey: Uint8Array,
    serverurl: string,
    serverPubKey: Uint8Array,
    features: string[],
  ) {
    this.privateKey = privateKey;
    this.publicKey = publicKey;
    this.serverurl = serverurl;
    this.serverPubKey = serverPubKey;
    this.features = features;
  }

  public static async new(
    serverurl: URL
  ) {
    let privateKey = p256.utils.randomPrivateKey();
    let publicKey = p256.getPublicKey(privateKey);
    let ExpectServerfingerPrint = serverurl.username

    serverurl.username = ""

    let serverConf = await Client.checkService(serverurl.toString());

    if (serverConf.result.version) {
      if (compareVersions(serverConf.result.version, '0.0.13') < 0) {
        throw 'server version is too old: ' + serverConf.result.version;
      }
    } else {
      throw 'cant get server version';
    }

    let serverPubKey;

    if (serverConf.result.pubkey) {
      let serverPubKeyHex = serverConf.result.pubkey;
      let serverFingerPrint = bytesToHex(sha256(hexToBytes(serverConf.result.pubkey)))

      if (ExpectServerfingerPrint == serverFingerPrint) {
        serverPubKey = hexToBytes(serverPubKeyHex);
      } else {
        throw 'server finger print does not match';
      }
      serverPubKey = hexToBytes(serverPubKeyHex);
    } else {
      throw 'json rpc error';
    }

    return new Client(privateKey, publicKey, serverurl.toString(), serverPubKey, serverConf.result.features);
  }

  static async checkService(
    serverurl: string
  ): Promise<JsonRpcResult<DiscoveryResult>> {
    let resp = await fetch(serverurl + 'discovery', {
      method: 'GET',
      mode: 'cors',
    });
    return resp.json();
  }

  static finger_print(sk: Uint8Array) {
    let digest = sha256(sk);
    let digest_hex = bytesToHex(digest);
    return digest_hex;
  }

  async deploy(params: DeployParams) {
    if (this.features.find((v) => v == "deploy") == undefined) {
      throw "server not implemented!"
    }

    let resp = await this.fetch({
      method: 'deploy',
      params: Object.values(params),
      jsonrpc: '2.0',
      id: 1,
    });
    return resp.json();
  }

  async execute(params: ExecuteParams) {
    if (this.features.find((v) => v == "execute") == undefined) {
      throw "server not implemented!"
    }
    let resp = await this.fetch({
      method: 'execute',
      params: Object.values(params),
      jsonrpc: '2.0',
      id: 1,
    });
    return resp.json();
  }

  async transfer(params: TransferParams) {
    if (this.features.find((v) => v == "transfer") == undefined) {
      throw "server not implemented!"
    }
    let resp = await this.fetch({
      method: 'transfer',
      params: Object.values(params),
      jsonrpc: '2.0',
      id: 1,
    });
    return resp.json();
  }

  async join(params: JoinParams) {
    if (this.features.find((v) => v == "join") == undefined) {
      throw "server not implemented!"
    }
    let resp = await this.fetch({
      method: 'join',
      params: Object.values(params),
      jsonrpc: '2.0',
      id: 1,
    });
    return resp.json();
  }

  async split(params: SplitParams) {
    if (this.features.find((v) => v == "split") == undefined) {
      throw "server not implemented!"
    }
    let resp = await this.fetch({
      method: 'split',
      params: Object.values(params),
      jsonrpc: '2.0',
      id: 1,
    });
    return resp.json();
  }

  async deployment_cost(params: DeploymentCostParams) {
    if (this.features.find((v) => v == "deployment_cost") == undefined) {
      throw "server not implemented!"
    }
    let resp = await this.fetch({
      method: 'deployment_cost',
      params: Object.values(params),
      jsonrpc: '2.0',
      id: 1,
    });
    return resp.json();
  }

  async execution_cost(params: ExecutionCostParams) {
    if (this.features.find((v) => v == "execution_costv2") == undefined) {
      throw "server not implemented!"
    }
    let resp = await this.fetch({
      method: 'execution_costv2',
      params: Object.values(params),
      jsonrpc: '2.0',
      id: 1,
    });
    return resp.json();
  }

  async decrypt_records(params: DecryptRecordsParams) {
    if (this.features.find((v) => v == "decrypt_records") == undefined) {
      throw "server not implemented!"
    }
    let resp = await this.fetch({
      method: 'decrypt_records',
      params: Object.values(params),
      jsonrpc: '2.0',
      id: 1,
    });
    return resp.json();
  }

  async transaction_from_authorization(params: TransactionFromAuthorizationParams) {
    if (this.features.find((v) => v == "transaction_from_authorization") == undefined) {
      throw "server not implemented!"
    }
    let resp = await this.fetch({
      method: 'transaction_from_authorization',
      params: Object.values(params),
      jsonrpc: '2.0',
      id: 1,
    });
    return resp.json();
  }

  async deploy_from_authorization(params: DeployFromAuthorizationParams) {
    if (this.features.find((v) => v == "deploy_from_authorization") == undefined) {
      throw "server not implemented!"
    }
    let resp = await this.fetch({
      method: 'deploy_from_authorization',
      params: Object.values(params),
      jsonrpc: '2.0',
      id: 1,
    });
    return resp.json();
  }

  async fetch(body: JsonRpcRequest<any>): Promise<Response> {
    let body_json = JSON.stringify(body);

    let shared = p256.getSharedSecret(this.privateKey, this.serverPubKey);

    shared = shared.slice(1);

    let derived = hkdf(sha256, shared, undefined, undefined, 32);

    const encodedData = new TextEncoder().encode(body_json);
    let encryptedBody = await encryptData(encodedData, derived);

    let resp = fetch(this.serverurl, {
      method: 'POST',
      body: encryptedBody,
      mode: 'cors',
      headers: {
        'Content-Type': 'application/octet-stream',
        'Public-Key': bytesToHex(this.publicKey),
      },
    });

    return resp;
  }
}

async function encryptData(
  data: ArrayBuffer,
  key: Uint8Array
): Promise<ArrayBuffer> {
  let crypto;
  if (self) {
    crypto = self.crypto;
  } else if (window) {
    crypto = window.crypto;
  } else {
    crypto = globalThis.crypto;
  }
  let aeskey = await crypto.subtle.importKey(
    'raw',
    key,
    { name: 'AES-GCM' },
    false,
    ['encrypt', 'decrypt']
  );

  const iv = crypto.getRandomValues(new Uint8Array(12));
  const algorithm = { name: 'AES-GCM', iv: iv };
  const encryptedData = await crypto.subtle.encrypt(algorithm, aeskey, data);

  const encryptedBuffer = new Uint8Array(encryptedData);
  const result = new Uint8Array(iv.length + encryptedBuffer.length);
  result.set(iv);
  result.set(encryptedBuffer, iv.length);

  return result.buffer;
}

function compareVersions(version1: string, version2: string) {
  const parts1 = version1.split('.').map(Number);
  const parts2 = version2.split('.').map(Number);

  for (let i = 0; i < parts1.length; i++) {
    if (parts1[i] > parts2[i]) {
      return 1;
    } else if (parts1[i] < parts2[i]) {
      return -1;
    }
  }

  return 0;
}
