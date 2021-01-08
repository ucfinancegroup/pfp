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
    ApiError,
    ApiErrorFromJSON,
    ApiErrorToJSON,
    ItemIdResponse,
    ItemIdResponseFromJSON,
    ItemIdResponseToJSON,
    LinkTokenCreateResponse,
    LinkTokenCreateResponseFromJSON,
    LinkTokenCreateResponseToJSON,
    PlaidJWT,
    PlaidJWTFromJSON,
    PlaidJWTToJSON,
    PublicTokenExchangeRequest,
    PublicTokenExchangeRequestFromJSON,
    PublicTokenExchangeRequestToJSON,
} from '../models';

export interface PlaidLinkAccessRequest {
    publicTokenExchangeRequest: PublicTokenExchangeRequest;
}

export interface PlaidWebhookRequest {
    plaidVerification?: PlaidJWT;
}

/**
 * 
 */
export class PlaidApi extends runtime.BaseAPI {

    /**
     * to request link token for PlaidLink
     */
    async plaidLinkRaw(): Promise<runtime.ApiResponse<LinkTokenCreateResponse>> {
        const queryParameters: runtime.HTTPQuery = {};

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

        const queryParameters: runtime.HTTPQuery = {};

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
     * Where Plaid sends updates about items, transactions, etc https://plaid.com/docs/api/webhooks/
     */
    async plaidWebhookRaw(requestParameters: PlaidWebhookRequest): Promise<runtime.ApiResponse<void>> {
        const queryParameters: runtime.HTTPQuery = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (requestParameters.plaidVerification !== undefined && requestParameters.plaidVerification !== null) {
            headerParameters['Plaid-Verification'] = String(requestParameters.plaidVerification);
        }

        const response = await this.request({
            path: `/plaid/webhook`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Where Plaid sends updates about items, transactions, etc https://plaid.com/docs/api/webhooks/
     */
    async plaidWebhook(requestParameters: PlaidWebhookRequest): Promise<void> {
        await this.plaidWebhookRaw(requestParameters);
    }

}