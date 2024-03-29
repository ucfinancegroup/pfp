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
    Event,
    EventFromJSON,
    EventToJSON,
} from '../models';

/**
 * 
 */
export class EventApi extends runtime.BaseAPI {

    /**
     * Get example events
     */
    async getEventExamplesRaw(): Promise<runtime.ApiResponse<Array<Event>>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/event/examples`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => jsonValue.map(EventFromJSON));
    }

    /**
     * Get example events
     */
    async getEventExamples(): Promise<Array<Event>> {
        const response = await this.getEventExamplesRaw();
        return await response.value();
    }

}
