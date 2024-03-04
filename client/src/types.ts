export interface DeployParams {
  private_key: string;
  program: string;
  fee_record?: string;
  imports?: { [key: string]: string };
  priority_fee_in_microcredits?: number;
  query?: string;
}

export interface ExecuteParams {
  private_key: string;
  program_id: string;
  function: string;
  inputs: string[];
  fee_record?: string
  imports?: { [key: string]: string };
  fee?: number;
  query?: string;
}

export interface TransferParams {
  private_key: string;
  recipient: string;
  amount: number;
  function: 'private' | 'public' | 'private_to_public' | 'public_to_private';
  input_record?: string;
  fee_record?: string;
  fee?: number;
  query?: string;
}

export interface JoinParams {
  private_key: string;
  first_record: string;
  second_record: string;
  fee_record?: string;
  fee?: number;
  query?: string;
}

export interface SplitParams {
  private_key: string;
  record: string;
  amount: number;
  query?: string;
}

export interface DeploymentCostParams {
  program: String,
  imports?: { [key: string]: string },
}

export interface ExecutionCostParams {
  program_id: string,
  function: string,
  inputs: string[],
  query?: string,
}

export interface DecryptRecordsParams {
  view_key: string,
  records: string[],
}

export interface TransactionFromAuthorizationParams {
  program_id: string,
  execute_authorization_str: string,
  fee_authorization_str: string,
  query?: string,
}

export interface DeployFromAuthorizationParams {
  program: string,
  imports?: { [key: string]: string },
  owner_str: string,
  fee_authorization_str: string,
  query?: string,
}

export interface UpdateParams {
  version: string;
}

export interface JsonRpcResult<T> {
  jsonrpc: string;
  result: T;
  error: {};
  id: number;
}

export interface JsonRpcRequest<T> {
  jsonrpc: '2.0';
  method: string;
  params: T;
  id: 1;
}

export interface DiscoveryResult {
  version: string;
  features: string[];
  pubkey: string;
}
