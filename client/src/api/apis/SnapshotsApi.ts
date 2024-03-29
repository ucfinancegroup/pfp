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
    Snapshot,
    SnapshotFromJSON,
    SnapshotToJSON,
} from '../models';

/**
 * 
 */
export class SnapshotsApi extends runtime.BaseAPI {

    /**
     * Get all a user\'s snapshots
     */
    async getSnapshotsRaw(): Promise<runtime.ApiResponse<Snapshot>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/snapshots`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => SnapshotFromJSON(jsonValue));
    }

    /**
     * Get all a user\'s snapshots
     */
    async getSnapshots(): Promise<Snapshot> {
        const response = await this.getSnapshotsRaw();
        return await response.value();
    }

}
