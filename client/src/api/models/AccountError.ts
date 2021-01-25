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
 * @interface AccountError
 */
export interface AccountError {
    /**
     * 
     * @type {string}
     * @memberof AccountError
     */
    message: string;
}

export function AccountErrorFromJSON(json: any): AccountError {
    return AccountErrorFromJSONTyped(json, false);
}

export function AccountErrorFromJSONTyped(json: any, ignoreDiscriminator: boolean): AccountError {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'message': json['message'],
    };
}

export function AccountErrorToJSON(value?: AccountError | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'message': value.message,
    };
}

