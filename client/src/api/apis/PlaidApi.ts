/* tslint:disable */
/* eslint-disable */
/**
 * FinchAPI
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import * as runtime from '../runtime';
import {
    AccountsResponse,
    AccountsResponseFromJSON,
    AccountsResponseToJSON,
    ApiError,
    ApiErrorFromJSON,
    ApiErrorToJSON,
    ItemIdResponse,
    ItemIdResponseFromJSON,
    ItemIdResponseToJSON,
    LinkTokenCreateResponse,
    LinkTokenCreateResponseFromJSON,
    LinkTokenCreateResponseToJSON,
    PublicTokenExchangeRequest,
    PublicTokenExchangeRequestFromJSON,
    PublicTokenExchangeRequestToJSON,
    SetAccountAsHiddenPayload,
    SetAccountAsHiddenPayloadFromJSON,
    SetAccountAsHiddenPayloadToJSON,
} from '../models';

export interface DeleteAccountRequest {
    id: string;
}

export interface GetAccountsRequest {
    allOrUnhidden: GetAccountsAllOrUnhiddenEnum;
}

export interface PlaidLinkAccessRequest {
    publicTokenExchangeRequest: PublicTokenExchangeRequest;
}

export interface SetAccountAsHiddenRequest {
    setAccountAsHiddenPayload: SetAccountAsHiddenPayload;
}

/**
 * 
 */
export class PlaidApi extends runtime.BaseAPI {

    /**
     * Delete account with given item_id
     */
    async deleteAccountRaw(requestParameters: DeleteAccountRequest): Promise<runtime.ApiResponse<ItemIdResponse>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling deleteAccount.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/plaid/accounts/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => ItemIdResponseFromJSON(jsonValue));
    }

    /**
     * Delete account with given item_id
     */
    async deleteAccount(requestParameters: DeleteAccountRequest): Promise<ItemIdResponse> {
        const response = await this.deleteAccountRaw(requestParameters);
        return await response.value();
    }

    /**
     * Get all of user\'s connected accounts
     */
    async getAccountsRaw(requestParameters: GetAccountsRequest): Promise<runtime.ApiResponse<AccountsResponse>> {
        if (requestParameters.allOrUnhidden === null || requestParameters.allOrUnhidden === undefined) {
            throw new runtime.RequiredError('allOrUnhidden','Required parameter requestParameters.allOrUnhidden was null or undefined when calling getAccounts.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/plaid/accounts/{allOrUnhidden}`.replace(`{${"allOrUnhidden"}}`, encodeURIComponent(String(requestParameters.allOrUnhidden))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => AccountsResponseFromJSON(jsonValue));
    }

    /**
     * Get all of user\'s connected accounts
     */
    async getAccounts(requestParameters: GetAccountsRequest): Promise<AccountsResponse> {
        const response = await this.getAccountsRaw(requestParameters);
        return await response.value();
    }

    /**
     * to request link token for PlaidLink
     */
    async plaidLinkRaw(): Promise<runtime.ApiResponse<LinkTokenCreateResponse>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/plaid/link_token`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => LinkTokenCreateResponseFromJSON(jsonValue));
    }

    /**
     * to request link token for PlaidLink
     */
    async plaidLink(): Promise<LinkTokenCreateResponse> {
        const response = await this.plaidLinkRaw();
        return await response.value();
    }

    /**
     * For after a user does PlaidLink thru client
     */
    async plaidLinkAccessRaw(requestParameters: PlaidLinkAccessRequest): Promise<runtime.ApiResponse<ItemIdResponse>> {
        if (requestParameters.publicTokenExchangeRequest === null || requestParameters.publicTokenExchangeRequest === undefined) {
            throw new runtime.RequiredError('publicTokenExchangeRequest','Required parameter requestParameters.publicTokenExchangeRequest was null or undefined when calling plaidLinkAccess.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/plaid/public_token_exchange`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: PublicTokenExchangeRequestToJSON(requestParameters.publicTokenExchangeRequest),
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => ItemIdResponseFromJSON(jsonValue));
    }

    /**
     * For after a user does PlaidLink thru client
     */
    async plaidLinkAccess(requestParameters: PlaidLinkAccessRequest): Promise<ItemIdResponse> {
        const response = await this.plaidLinkAccessRaw(requestParameters);
        return await response.value();
    }

    /**
     * Hides or unhides an account
     */
    async setAccountAsHiddenRaw(requestParameters: SetAccountAsHiddenRequest): Promise<runtime.ApiResponse<AccountsResponse>> {
        if (requestParameters.setAccountAsHiddenPayload === null || requestParameters.setAccountAsHiddenPayload === undefined) {
            throw new runtime.RequiredError('setAccountAsHiddenPayload','Required parameter requestParameters.setAccountAsHiddenPayload was null or undefined when calling setAccountAsHidden.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/plaid/accounts/hide`,
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
            body: SetAccountAsHiddenPayloadToJSON(requestParameters.setAccountAsHiddenPayload),
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => AccountsResponseFromJSON(jsonValue));
    }

    /**
     * Hides or unhides an account
     */
    async setAccountAsHidden(requestParameters: SetAccountAsHiddenRequest): Promise<AccountsResponse> {
        const response = await this.setAccountAsHiddenRaw(requestParameters);
        return await response.value();
    }

}

/**
    * @export
    * @enum {string}
    */
export enum GetAccountsAllOrUnhiddenEnum {
    All = 'all',
    Unhidden = 'unhidden'
}
