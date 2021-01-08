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
    UserLocation,
    UserLocationFromJSON,
    UserLocationFromJSONTyped,
    UserLocationToJSON,
} from './';

/**
 * 
 * @export
 * @interface User
 */
export interface User {
    /**
     * 
     * @type {string}
     * @memberof User
     */
    email: string;
    /**
     * 
     * @type {UserLocation}
     * @memberof User
     */
    location: UserLocation;
    /**
     * 
     * @type {string}
     * @memberof User
     */
    first_name: string;
    /**
     * 
     * @type {string}
     * @memberof User
     */
    last_name: string;
    /**
     * 
     * @type {number}
     * @memberof User
     */
    income?: number;
    /**
     * 
     * @type {string}
     * @memberof User
     */
    device_url?: string;
}

export function UserFromJSON(json: any): User {
    return UserFromJSONTyped(json, false);
}

export function UserFromJSONTyped(json: any, ignoreDiscriminator: boolean): User {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'email': json['email'],
        'location': UserLocationFromJSON(json['location']),
        'first_name': json['first_name'],
        'last_name': json['last_name'],
        'income': !exists(json, 'income') ? undefined : json['income'],
        'device_url': !exists(json, 'device_url') ? undefined : json['device_url'],
    };
}

export function UserToJSON(value?: User | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'email': value.email,
        'location': UserLocationToJSON(value.location),
        'first_name': value.first_name,
        'last_name': value.last_name,
        'income': value.income,
        'device_url': value.device_url,
    };
}


