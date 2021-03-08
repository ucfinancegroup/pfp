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
 * @interface Money
 */
export interface Money {
    /**
     * 
     * @type {number}
     * @memberof Money
     */
    amount: number;
}

export function MoneyFromJSON(json: any): Money {
    return MoneyFromJSONTyped(json, false);
}

export function MoneyFromJSONTyped(json: any, ignoreDiscriminator: boolean): Money {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'amount': json['amount'],
    };
}

export function MoneyToJSON(value?: Money | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'amount': value.amount,
    };
}


