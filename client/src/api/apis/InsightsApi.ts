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


import * as runtime from '../runtime';
import {
    ApiError,
    ApiErrorFromJSON,
    ApiErrorToJSON,
    Insight,
    InsightFromJSON,
    InsightToJSON,
} from '../models';

export interface DismissInsightRequest {
    id: string;
}

/**
 * 
 */
export class InsightsApi extends runtime.BaseAPI {

    /**
     * Dismiss an insight
     */
    async dismissInsightRaw(requestParameters: DismissInsightRequest): Promise<runtime.ApiResponse<Insight>> {
        if (requestParameters.id === null || requestParameters.id === undefined) {
            throw new runtime.RequiredError('id','Required parameter requestParameters.id was null or undefined when calling dismissInsight.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/insight/{id}/dismiss`.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters.id))),
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => InsightFromJSON(jsonValue));
    }

    /**
     * Dismiss an insight
     */
    async dismissInsight(requestParameters: DismissInsightRequest): Promise<Insight> {
        const response = await this.dismissInsightRaw(requestParameters);
        return await response.value();
    }

    /**
     * Get example insights
     */
    async getInsightExamplesRaw(): Promise<runtime.ApiResponse<Array<Insight>>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/insights/examples`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => jsonValue.map(InsightFromJSON));
    }

    /**
     * Get example insights
     */
    async getInsightExamples(): Promise<Array<Insight>> {
        const response = await this.getInsightExamplesRaw();
        return await response.value();
    }

    /**
     * Get all a user\'s (non-dismissed) insights
     */
    async getInsightsRaw(): Promise<runtime.ApiResponse<Array<Insight>>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/insights`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => jsonValue.map(InsightFromJSON));
    }

    /**
     * Get all a user\'s (non-dismissed) insights
     */
    async getInsights(): Promise<Array<Insight>> {
        const response = await this.getInsightsRaw();
        return await response.value();
    }

}
