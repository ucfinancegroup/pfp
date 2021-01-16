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
    GoalMetric,
    GoalMetricFromJSON,
    GoalMetricFromJSONTyped,
    GoalMetricToJSON,
} from './';

/**
 *
 * @export
 * @interface GoalNewPayload
 */
export interface GoalNewPayload {
    /**
     *
     * @type {string}
     * @memberof GoalNewPayload
     */
    name: string;
    /**
     *
     * @type {number}
     * @memberof GoalNewPayload
     */
    start: number;
    /**
     *
     * @type {number}
     * @memberof GoalNewPayload
     */
    end: number;
    /**
     *
     * @type {number}
     * @memberof GoalNewPayload
     */
    threshold: number;
    /**
     *
     * @type {GoalMetric}
     * @memberof GoalNewPayload
     */
    metric: GoalMetric;
}

export function GoalNewPayloadFromJSON(json: any): GoalNewPayload {
    return GoalNewPayloadFromJSONTyped(json, false);
}

export function GoalNewPayloadFromJSONTyped(json: any, ignoreDiscriminator: boolean): GoalNewPayload {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {

        'name': json['name'],
        'start': json['start'],
        'end': json['end'],
        'threshold': json['threshold'],
        'metric': GoalMetricFromJSON(json['metric']),
    };
}

export function GoalNewPayloadToJSON(value?: GoalNewPayload | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {

        'name': value.name,
        'start': value.start,
        'end': value.end,
        'threshold': value.threshold,
        'metric': GoalMetricToJSON(value.metric),
    };
}


