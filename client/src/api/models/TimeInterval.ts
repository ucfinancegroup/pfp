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
 * @interface TimeInterval
 */
export interface TimeInterval {
    /**
     * 
     * @type {string}
     * @memberof TimeInterval
     */
    typ: TimeIntervalTypEnum;
    /**
     * 
     * @type {number}
     * @memberof TimeInterval
     */
    content: number;
}

/**
* @export
* @enum {string}
*/
export enum TimeIntervalTypEnum {
    Monthly = 'monthly',
    Annually = 'annually',
    Daily = 'daily',
    Weekly = 'weekly'
}

export function TimeIntervalFromJSON(json: any): TimeInterval {
    return TimeIntervalFromJSONTyped(json, false);
}

export function TimeIntervalFromJSONTyped(json: any, ignoreDiscriminator: boolean): TimeInterval {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'typ': json['typ'],
        'content': json['content'],
    };
}

export function TimeIntervalToJSON(value?: TimeInterval | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'typ': value.typ,
        'content': value.content,
    };
}


