// automatically generated by the FlatBuffers compiler, do not modify

import { WasmContractV1, WasmContractV1T } from '../common/wasm-contract-v1.js';


export enum ContractType {
  NONE = 0,
  WasmContractV1 = 1
}

export function unionToContractType(
  type: ContractType,
  accessor: (obj:WasmContractV1) => WasmContractV1|null
): WasmContractV1|null {
  switch(ContractType[type]) {
    case 'NONE': return null; 
    case 'WasmContractV1': return accessor(new WasmContractV1())! as WasmContractV1;
    default: return null;
  }
}

export function unionListToContractType(
  type: ContractType, 
  accessor: (index: number, obj:WasmContractV1) => WasmContractV1|null, 
  index: number
): WasmContractV1|null {
  switch(ContractType[type]) {
    case 'NONE': return null; 
    case 'WasmContractV1': return accessor(index, new WasmContractV1())! as WasmContractV1;
    default: return null;
  }
}