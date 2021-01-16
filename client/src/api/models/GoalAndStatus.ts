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
    Goal,
    GoalFromJSON,
    GoalFromJSONTyped,
    GoalToJSON,
} from './';

/**
 *
 * @export
 * @interface GoalAndStatus
 */
export interface GoalAndStatus {
    /**
     *
     * @type {Goal}
     * @memberof GoalAndStatus
     */
    goal: Goal;
    /**
     *
     * @type {number}
     * @memberof GoalAndStatus
     */
    progress: number;
}

export function GoalAndStatusFromJSON(json: any): GoalAndStatus {
    return GoalAndStatusFromJSONTyped(json, false);
}

export function GoalAndStatusFromJSONTyped(json: any, ignoreDiscriminator: boolean): GoalAndStatus {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {

        'goal': GoalFromJSON(json['goal']),
        'progress': json['progress'],
    };
}

export function GoalAndStatusToJSON(value?: GoalAndStatus | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {

        'goal': GoalToJSON(value.goal),
        'progress': value.progress,
    };
}


