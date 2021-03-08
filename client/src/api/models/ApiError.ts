/* tslint:disable */
/* eslint-disable */
/**
 * FinchAPI
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
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
 * @interface ApiError
 */
export interface ApiError {
    /**
     * 
     * @type {number}
     * @memberof ApiError
     */
    code: number;
    /**
     * 
     * @type {string}
     * @memberof ApiError
     */
    message: string;
}

export function ApiErrorFromJSON(json: any): ApiError {
    return ApiErrorFromJSONTyped(json, false);
}

export function ApiErrorFromJSONTyped(json: any, ignoreDiscriminator: boolean): ApiError {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'code': json['code'],
        'message': json['message'],
    };
}

export function ApiErrorToJSON(value?: ApiError | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'code': value.code,
        'message': value.message,
    };
}


