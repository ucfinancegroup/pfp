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
    GoalMetric,
    GoalMetricFromJSON,
    GoalMetricFromJSONTyped,
    GoalMetricToJSON,
    MongoObjectID,
    MongoObjectIDFromJSON,
    MongoObjectIDFromJSONTyped,
    MongoObjectIDToJSON,
} from './';

/**
 * 
 * @export
 * @interface Goal
 */
export interface Goal {
    /**
     * 
     * @type {MongoObjectID}
     * @memberof Goal
     */
    _id: MongoObjectID;
    /**
     * 
     * @type {string}
     * @memberof Goal
     */
    name: string;
    /**
     * 
     * @type {number}
     * @memberof Goal
     */
    start: number;
    /**
     * 
     * @type {number}
     * @memberof Goal
     */
    end: number;
    /**
     * 
     * @type {number}
     * @memberof Goal
     */
    threshold: number;
    /**
     * 
     * @type {GoalMetric}
     * @memberof Goal
     */
    metric: GoalMetric;
}

export function GoalFromJSON(json: any): Goal {
    return GoalFromJSONTyped(json, false);
}

export function GoalFromJSONTyped(json: any, ignoreDiscriminator: boolean): Goal {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        '_id': MongoObjectIDFromJSON(json['_id']),
        'name': json['name'],
        'start': json['start'],
        'end': json['end'],
        'threshold': json['threshold'],
        'metric': GoalMetricFromJSON(json['metric']),
    };
}

export function GoalToJSON(value?: Goal | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        '_id': MongoObjectIDToJSON(value._id),
        'name': value.name,
        'start': value.start,
        'end': value.end,
        'threshold': value.threshold,
        'metric': GoalMetricToJSON(value.metric),
    };
}


