// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { UseQueryOptions, useQuery, useMutation, UseMutationOptions } from '@tanstack/react-query'
import { ExecuteResult } from '@cosmjs/cosmwasm-stargate'
import { StdFee } from '@cosmjs/amino'
import {
  Decimal,
  InstantiateMsg,
  ExecuteMsg,
  OwnerUpdate,
  AssetParamsUpdate,
  HlsAssetTypeForString,
  Uint128,
  VaultConfigUpdate,
  EmergencyUpdate,
  CmEmergencyUpdate,
  RedBankEmergencyUpdate,
  AssetParamsBaseForString,
  CmSettingsForString,
  HlsParamsBaseForString,
  LiquidationBonus,
  RedBankSettings,
  VaultConfigBaseForString,
  Coin,
  QueryMsg,
  HlsAssetTypeForAddr,
  Addr,
  ArrayOfAssetParamsBaseForAddr,
  AssetParamsBaseForAddr,
  CmSettingsForAddr,
  HlsParamsBaseForAddr,
  ArrayOfVaultConfigBaseForAddr,
  VaultConfigBaseForAddr,
  ConfigResponse,
  OwnerResponse,
  TotalDepositResponse,
} from './MarsParams.types'
import { MarsParamsQueryClient, MarsParamsClient } from './MarsParams.client'
export const marsParamsQueryKeys = {
  contract: [
    {
      contract: 'marsParams',
    },
  ] as const,
  address: (contractAddress: string | undefined) =>
    [{ ...marsParamsQueryKeys.contract[0], address: contractAddress }] as const,
  owner: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [{ ...marsParamsQueryKeys.address(contractAddress)[0], method: 'owner', args }] as const,
  config: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [{ ...marsParamsQueryKeys.address(contractAddress)[0], method: 'config', args }] as const,
  assetParams: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [{ ...marsParamsQueryKeys.address(contractAddress)[0], method: 'asset_params', args }] as const,
  allAssetParams: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...marsParamsQueryKeys.address(contractAddress)[0], method: 'all_asset_params', args },
    ] as const,
  vaultConfig: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [{ ...marsParamsQueryKeys.address(contractAddress)[0], method: 'vault_config', args }] as const,
  allVaultConfigs: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...marsParamsQueryKeys.address(contractAddress)[0], method: 'all_vault_configs', args },
    ] as const,
  targetHealthFactor: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...marsParamsQueryKeys.address(contractAddress)[0], method: 'target_health_factor', args },
    ] as const,
  totalDeposit: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...marsParamsQueryKeys.address(contractAddress)[0], method: 'total_deposit', args },
    ] as const,
}
export interface MarsParamsReactQuery<TResponse, TData = TResponse> {
  client: MarsParamsQueryClient | undefined
  options?: Omit<
    UseQueryOptions<TResponse, Error, TData>,
    "'queryKey' | 'queryFn' | 'initialData'"
  > & {
    initialData?: undefined
  }
}
export interface MarsParamsTotalDepositQuery<TData>
  extends MarsParamsReactQuery<TotalDepositResponse, TData> {
  args: {
    denom: string
  }
}
export function useMarsParamsTotalDepositQuery<TData = TotalDepositResponse>({
  client,
  args,
  options,
}: MarsParamsTotalDepositQuery<TData>) {
  return useQuery<TotalDepositResponse, Error, TData>(
    marsParamsQueryKeys.totalDeposit(client?.contractAddress, args),
    () =>
      client
        ? client.totalDeposit({
            denom: args.denom,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsParamsTargetHealthFactorQuery<TData>
  extends MarsParamsReactQuery<Decimal, TData> {}
export function useMarsParamsTargetHealthFactorQuery<TData = Decimal>({
  client,
  options,
}: MarsParamsTargetHealthFactorQuery<TData>) {
  return useQuery<Decimal, Error, TData>(
    marsParamsQueryKeys.targetHealthFactor(client?.contractAddress),
    () => (client ? client.targetHealthFactor() : Promise.reject(new Error('Invalid client'))),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsParamsAllVaultConfigsQuery<TData>
  extends MarsParamsReactQuery<ArrayOfVaultConfigBaseForAddr, TData> {
  args: {
    limit?: number
    startAfter?: string
  }
}
export function useMarsParamsAllVaultConfigsQuery<TData = ArrayOfVaultConfigBaseForAddr>({
  client,
  args,
  options,
}: MarsParamsAllVaultConfigsQuery<TData>) {
  return useQuery<ArrayOfVaultConfigBaseForAddr, Error, TData>(
    marsParamsQueryKeys.allVaultConfigs(client?.contractAddress, args),
    () =>
      client
        ? client.allVaultConfigs({
            limit: args.limit,
            startAfter: args.startAfter,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsParamsVaultConfigQuery<TData>
  extends MarsParamsReactQuery<VaultConfigBaseForAddr, TData> {
  args: {
    address: string
  }
}
export function useMarsParamsVaultConfigQuery<TData = VaultConfigBaseForAddr>({
  client,
  args,
  options,
}: MarsParamsVaultConfigQuery<TData>) {
  return useQuery<VaultConfigBaseForAddr, Error, TData>(
    marsParamsQueryKeys.vaultConfig(client?.contractAddress, args),
    () =>
      client
        ? client.vaultConfig({
            address: args.address,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsParamsAllAssetParamsQuery<TData>
  extends MarsParamsReactQuery<ArrayOfAssetParamsBaseForAddr, TData> {
  args: {
    limit?: number
    startAfter?: string
  }
}
export function useMarsParamsAllAssetParamsQuery<TData = ArrayOfAssetParamsBaseForAddr>({
  client,
  args,
  options,
}: MarsParamsAllAssetParamsQuery<TData>) {
  return useQuery<ArrayOfAssetParamsBaseForAddr, Error, TData>(
    marsParamsQueryKeys.allAssetParams(client?.contractAddress, args),
    () =>
      client
        ? client.allAssetParams({
            limit: args.limit,
            startAfter: args.startAfter,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsParamsAssetParamsQuery<TData>
  extends MarsParamsReactQuery<AssetParamsBaseForAddr, TData> {
  args: {
    denom: string
  }
}
export function useMarsParamsAssetParamsQuery<TData = AssetParamsBaseForAddr>({
  client,
  args,
  options,
}: MarsParamsAssetParamsQuery<TData>) {
  return useQuery<AssetParamsBaseForAddr, Error, TData>(
    marsParamsQueryKeys.assetParams(client?.contractAddress, args),
    () =>
      client
        ? client.assetParams({
            denom: args.denom,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsParamsConfigQuery<TData> extends MarsParamsReactQuery<ConfigResponse, TData> {}
export function useMarsParamsConfigQuery<TData = ConfigResponse>({
  client,
  options,
}: MarsParamsConfigQuery<TData>) {
  return useQuery<ConfigResponse, Error, TData>(
    marsParamsQueryKeys.config(client?.contractAddress),
    () => (client ? client.config() : Promise.reject(new Error('Invalid client'))),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsParamsOwnerQuery<TData> extends MarsParamsReactQuery<OwnerResponse, TData> {}
export function useMarsParamsOwnerQuery<TData = OwnerResponse>({
  client,
  options,
}: MarsParamsOwnerQuery<TData>) {
  return useQuery<OwnerResponse, Error, TData>(
    marsParamsQueryKeys.owner(client?.contractAddress),
    () => (client ? client.owner() : Promise.reject(new Error('Invalid client'))),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsParamsEmergencyUpdateMutation {
  client: MarsParamsClient
  msg: EmergencyUpdate
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMarsParamsEmergencyUpdateMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, MarsParamsEmergencyUpdateMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, MarsParamsEmergencyUpdateMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) =>
      client.emergencyUpdate(msg, fee, memo, funds),
    options,
  )
}
export interface MarsParamsUpdateVaultConfigMutation {
  client: MarsParamsClient
  msg: VaultConfigUpdate
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMarsParamsUpdateVaultConfigMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, MarsParamsUpdateVaultConfigMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, MarsParamsUpdateVaultConfigMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) =>
      client.updateVaultConfig(msg, fee, memo, funds),
    options,
  )
}
export interface MarsParamsUpdateAssetParamsMutation {
  client: MarsParamsClient
  msg: AssetParamsUpdate
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMarsParamsUpdateAssetParamsMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, MarsParamsUpdateAssetParamsMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, MarsParamsUpdateAssetParamsMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) =>
      client.updateAssetParams(msg, fee, memo, funds),
    options,
  )
}
export interface MarsParamsUpdateTargetHealthFactorMutation {
  client: MarsParamsClient
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMarsParamsUpdateTargetHealthFactorMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, MarsParamsUpdateTargetHealthFactorMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, MarsParamsUpdateTargetHealthFactorMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) =>
      client.updateTargetHealthFactor(msg, fee, memo, funds),
    options,
  )
}
export interface MarsParamsUpdateConfigMutation {
  client: MarsParamsClient
  msg: {
    addressProvider?: string
  }
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMarsParamsUpdateConfigMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, MarsParamsUpdateConfigMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, MarsParamsUpdateConfigMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) =>
      client.updateConfig(msg, fee, memo, funds),
    options,
  )
}
export interface MarsParamsUpdateOwnerMutation {
  client: MarsParamsClient
  msg: OwnerUpdate
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMarsParamsUpdateOwnerMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, MarsParamsUpdateOwnerMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, MarsParamsUpdateOwnerMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) => client.updateOwner(msg, fee, memo, funds),
    options,
  )
}
