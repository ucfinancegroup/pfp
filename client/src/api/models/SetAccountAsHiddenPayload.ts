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
 * @interface SetAccountAsHiddenPayload
 */
export interface SetAccountAsHiddenPayload {
    /**
     * 
     * @type {string}
     * @memberof SetAccountAsHiddenPayload
     */
    item_id: string;
    /**
     * 
     * @type {string}
     * @memberof SetAccountAsHiddenPayload
     */
    account_id: string;
    /**
     * 
     * @type {boolean}
     * @memberof SetAccountAsHiddenPayload
     */
    hide_or_not: boolean;
}

export function SetAccountAsHiddenPayloadFromJSON(json: any): SetAccountAsHiddenPayload {
    return SetAccountAsHiddenPayloadFromJSONTyped(json, false);
}

export function SetAccountAsHiddenPayloadFromJSONTyped(json: any, ignoreDiscriminator: boolean): SetAccountAsHiddenPayload {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'item_id': json['item_id'],
        'account_id': json['account_id'],
        'hide_or_not': json['hide_or_not'],
    };
}

export function SetAccountAsHiddenPayloadToJSON(value?: SetAccountAsHiddenPayload | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'item_id': value.item_id,
        'account_id': value.account_id,
        'hide_or_not': value.hide_or_not,
    };
}

