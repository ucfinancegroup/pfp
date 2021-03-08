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
import {
    Money,
    MoneyFromJSON,
    MoneyFromJSONTyped,
    MoneyToJSON,
} from './';

/**
 * Daily values of user's net worth
 * @export
 * @interface TimeSeriesEntry
 */
export interface TimeSeriesEntry {
    /**
     * 
     * @type {number}
     * @memberof TimeSeriesEntry
     */
    date: number;
    /**
     * 
     * @type {Money}
     * @memberof TimeSeriesEntry
     */
    net_worth: Money;
}

export function TimeSeriesEntryFromJSON(json: any): TimeSeriesEntry {
    return TimeSeriesEntryFromJSONTyped(json, false);
}

export function TimeSeriesEntryFromJSONTyped(json: any, ignoreDiscriminator: boolean): TimeSeriesEntry {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'date': json['date'],
        'net_worth': MoneyFromJSON(json['net_worth']),
    };
}

export function TimeSeriesEntryToJSON(value?: TimeSeriesEntry | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'date': value.date,
        'net_worth': MoneyToJSON(value.net_worth),
    };
}


