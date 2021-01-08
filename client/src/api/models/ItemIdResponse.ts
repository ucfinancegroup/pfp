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
 * @interface ItemIdResponse
 */
export interface ItemIdResponse {
    /**
     * Item ID for newly added account
     * @type {string}
     * @memberof ItemIdResponse
     */
    item_id: string;
}

export function ItemIdResponseFromJSON(json: any): ItemIdResponse {
    return ItemIdResponseFromJSONTyped(json, false);
}

export function ItemIdResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): ItemIdResponse {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'item_id': json['item_id'],
    };
}

export function ItemIdResponseToJSON(value?: ItemIdResponse | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'item_id': value.item_id,
    };
}

