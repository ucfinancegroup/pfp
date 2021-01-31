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
import {
    Asset,
    AssetFromJSON,
    AssetFromJSONTyped,
    AssetToJSON,
} from './';

/**
 * 
 * @export
 * @interface AssetChange
 */
export interface AssetChange {
    /**
     * 
     * @type {Asset}
     * @memberof AssetChange
     */
    asset: Asset;
    /**
     * 
     * @type {number}
     * @memberof AssetChange
     */
    change: number;
}

export function AssetChangeFromJSON(json: any): AssetChange {
    return AssetChangeFromJSONTyped(json, false);
}

export function AssetChangeFromJSONTyped(json: any, ignoreDiscriminator: boolean): AssetChange {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'asset': AssetFromJSON(json['asset']),
        'change': json['change'],
    };
}

export function AssetChangeToJSON(value?: AssetChange | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'asset': AssetToJSON(value.asset),
        'change': value.change,
    };
}

