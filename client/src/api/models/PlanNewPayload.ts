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
    Allocation,
    AllocationFromJSON,
    AllocationFromJSONTyped,
    AllocationToJSON,
    Event,
    EventFromJSON,
    EventFromJSONTyped,
    EventToJSON,
    Recurring,
    RecurringFromJSON,
    RecurringFromJSONTyped,
    RecurringToJSON,
} from './';

/**
 * 
 * @export
 * @interface PlanNewPayload
 */
export interface PlanNewPayload {
    /**
     * 
     * @type {string}
     * @memberof PlanNewPayload
     */
    name: string;
    /**
     * 
     * @type {Array<Recurring>}
     * @memberof PlanNewPayload
     */
    recurrings: Array<Recurring>;
    /**
     * 
     * @type {Array<Allocation>}
     * @memberof PlanNewPayload
     */
    allocations: Array<Allocation>;
    /**
     * 
     * @type {Array<Event>}
     * @memberof PlanNewPayload
     */
    events: Array<Event>;
}

export function PlanNewPayloadFromJSON(json: any): PlanNewPayload {
    return PlanNewPayloadFromJSONTyped(json, false);
}

export function PlanNewPayloadFromJSONTyped(json: any, ignoreDiscriminator: boolean): PlanNewPayload {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'name': json['name'],
        'recurrings': ((json['recurrings'] as Array<any>).map(RecurringFromJSON)),
        'allocations': ((json['allocations'] as Array<any>).map(AllocationFromJSON)),
        'events': ((json['events'] as Array<any>).map(EventFromJSON)),
    };
}

export function PlanNewPayloadToJSON(value?: PlanNewPayload | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'name': value.name,
        'recurrings': ((value.recurrings as Array<any>).map(RecurringToJSON)),
        'allocations': ((value.allocations as Array<any>).map(AllocationToJSON)),
        'events': ((value.events as Array<any>).map(EventToJSON)),
    };
}


