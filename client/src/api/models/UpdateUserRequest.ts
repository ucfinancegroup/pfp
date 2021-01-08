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
 * @interface UpdateUserRequest
 */
export interface UpdateUserRequest {
    /**
     * 
     * @type {string}
     * @memberof UpdateUserRequest
     */
    password?: string;
    /**
     * 
     * @type {string}
     * @memberof UpdateUserRequest
     */
    first_name?: string;
    /**
     * 
     * @type {string}
     * @memberof UpdateUserRequest
     */
    last_name?: string;
    /**
     * 
     * @type {number}
     * @memberof UpdateUserRequest
     */
    income?: number;
}

export function UpdateUserRequestFromJSON(json: any): UpdateUserRequest {
    return UpdateUserRequestFromJSONTyped(json, false);
}

export function UpdateUserRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): UpdateUserRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'password': !exists(json, 'password') ? undefined : json['password'],
        'first_name': !exists(json, 'first_name') ? undefined : json['first_name'],
        'last_name': !exists(json, 'last_name') ? undefined : json['last_name'],
        'income': !exists(json, 'income') ? undefined : json['income'],
    };
}

export function UpdateUserRequestToJSON(value?: UpdateUserRequest | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'password': value.password,
        'first_name': value.first_name,
        'last_name': value.last_name,
        'income': value.income,
    };
}

