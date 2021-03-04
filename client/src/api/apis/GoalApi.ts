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


import * as runtime from '../runtime';
import {
    ApiError,
    ApiErrorFromJSON,
    ApiErrorToJSON,
    GoalAndStatus,
    GoalAndStatusFromJSON,
    GoalAndStatusToJSON,
    GoalNewPayload,
    GoalNewPayloadFromJSON,
    GoalNewPayloadToJSON,
} from '../models';

export interface DeleteGoalRequest {
    id: string;
}

export interface GetGoalRequest {
    id: string;
}

export interface NewGoalRequest {
    goalNewPayload: GoalNewPayload;
}

export interface UpdateGoalRequest {
    id: string;
    goalNewPayload: GoalNewPayload;
}

/**
 * 
 */
export class GoalApi extends runtime.BaseAPI {

    /**
     * Delete one specific goal by id
     */
    async deleteGoalRaw(requestParameters: DeleteGoalRequest): Promise<runtime.ApiResponse<GoalAndStatus>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling deleteGoal.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/goal/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => GoalAndStatusFromJSON(jsonValue));
    }

    /**
     * Delete one specific goal by id
     */
    async deleteGoal(requestParameters: DeleteGoalRequest): Promise<GoalAndStatus> {
        const response = await this.deleteGoalRaw(requestParameters);
        return await response.value();
    }

    /**
     * Get one specific goal by id
     */
    async getGoalRaw(requestParameters: GetGoalRequest): Promise<runtime.ApiResponse<GoalAndStatus>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling getGoal.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/goal/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => GoalAndStatusFromJSON(jsonValue));
    }

    /**
     * Get one specific goal by id
     */
    async getGoal(requestParameters: GetGoalRequest): Promise<GoalAndStatus> {
        const response = await this.getGoalRaw(requestParameters);
        return await response.value();
    }

    /**
     * Get example Goals
     */
    async getGoalExamplesRaw(): Promise<runtime.ApiResponse<Array<GoalNewPayload>>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/goal/examples`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => jsonValue.map(GoalNewPayloadFromJSON));
    }

    /**
     * Get example Goals
     */
    async getGoalExamples(): Promise<Array<GoalNewPayload>> {
        const response = await this.getGoalExamplesRaw();
        return await response.value();
    }

    /**
     * Get all of a user\'s goals
     */
    async getGoalsRaw(): Promise<runtime.ApiResponse<Array<GoalAndStatus>>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/goals`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => jsonValue.map(GoalAndStatusFromJSON));
    }

    /**
     * Get all of a user\'s goals
     */
    async getGoals(): Promise<Array<GoalAndStatus>> {
        const response = await this.getGoalsRaw();
        return await response.value();
    }

    /**
     * Creates a new goal for the user
     */
    async newGoalRaw(requestParameters: NewGoalRequest): Promise<runtime.ApiResponse<GoalAndStatus>> {
        if (requestParameters.goalNewPayload === null || requestParameters.goalNewPayload === undefined) {
            throw new runtime.RequiredError('goalNewPayload','Required parameter requestParameters.goalNewPayload was null or undefined when calling newGoal.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/goal/new`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: GoalNewPayloadToJSON(requestParameters.goalNewPayload),
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => GoalAndStatusFromJSON(jsonValue));
    }

    /**
     * Creates a new goal for the user
     */
    async newGoal(requestParameters: NewGoalRequest): Promise<GoalAndStatus> {
        const response = await this.newGoalRaw(requestParameters);
        return await response.value();
    }

    /**
     * Update one specific goal by id
     */
    async updateGoalRaw(requestParameters: UpdateGoalRequest): Promise<runtime.ApiResponse<GoalAndStatus>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling updateGoal.');
        }

        if (requestParameters.goalNewPayload === null || requestParameters.goalNewPayload === undefined) {
            throw new runtime.RequiredError('goalNewPayload','Required parameter requestParameters.goalNewPayload was null or undefined when calling updateGoal.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/goal/{id}`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
            body: GoalNewPayloadToJSON(requestParameters.goalNewPayload),
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => GoalAndStatusFromJSON(jsonValue));
    }

    /**
     * Update one specific goal by id
     */
    async updateGoal(requestParameters: UpdateGoalRequest): Promise<GoalAndStatus> {
        const response = await this.updateGoalRaw(requestParameters);
        return await response.value();
    }

}
