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
 * @interface Account
 */
export interface Account {
    /**
     *
     * @type {string}
     * @memberof Account
     */
    item_id: string;
    /**
     *
     * @type {string}
     * @memberof Account
     */
    account_id: string;
    /**
     *
     * @type {string}
     * @memberof Account
     */
    name: string;
    /**
     *
     * @type {number}
     * @memberof Account
     */
    balance: number;
}

export function AccountFromJSON(json: any): Account {
    return AccountFromJSONTyped(json, false);
}

export function AccountFromJSONTyped(json: any, ignoreDiscriminator: boolean): Account {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {

        'item_id': json['item_id'],
        'account_id': json['account_id'],
        'name': json['name'],
        'balance': json['balance'],
    };
}

export function AccountToJSON(value?: Account | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {

        'item_id': value.item_id,
        'account_id': value.account_id,
        'name': value.name,
        'balance': value.balance,
    };
}


