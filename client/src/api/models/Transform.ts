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
    AssetToPercentMapping,
    AssetToPercentMappingFromJSON,
    AssetToPercentMappingFromJSONTyped,
    AssetToPercentMappingToJSON,
} from './';

/**
 * 
 * @export
 * @interface Transform
 */
export interface Transform {
    /**
     * 
     * @type {number}
     * @memberof Transform
     */
    trigger: number;
    /**
     * 
     * @type {{ [key: string]: AssetToPercentMapping; }}
     * @memberof Transform
     */
    change: { [key: string]: AssetToPercentMapping; };
}

export function TransformFromJSON(json: any): Transform {
    return TransformFromJSONTyped(json, false);
}

export function TransformFromJSONTyped(json: any, ignoreDiscriminator: boolean): Transform {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'trigger': json['trigger'],
        'change': (mapValues(json['change'], AssetToPercentMappingFromJSON)),
    };
}

export function TransformToJSON(value?: Transform | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'trigger': value.trigger,
        'change': (mapValues(value.change, AssetToPercentMappingToJSON)),
    };
}

