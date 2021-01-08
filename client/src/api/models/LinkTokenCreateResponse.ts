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

import { exists, mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface LinkTokenCreateResponse
 */
export interface LinkTokenCreateResponse {
    /**
     * 
     * @type {string}
     * @memberof LinkTokenCreateResponse
     */
    link_token: string;
    /**
     * 
     * @type {string}
     * @memberof LinkTokenCreateResponse
     */
    expiration: string;
    /**
     * 
     * @type {string}
     * @memberof LinkTokenCreateResponse
     */
    request_id?: string;
}

export function LinkTokenCreateResponseFromJSON(json: any): LinkTokenCreateResponse {
    return LinkTokenCreateResponseFromJSONTyped(json, false);
}

export function LinkTokenCreateResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): LinkTokenCreateResponse {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'link_token': json['link_token'],
        'expiration': json['expiration'],
        'request_id': !exists(json, 'request_id') ? undefined : json['request_id'],
    };
}

export function LinkTokenCreateResponseToJSON(value?: LinkTokenCreateResponse | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'link_token': value.link_token,
        'expiration': value.expiration,
        'request_id': value.request_id,
    };
}

