declare module 'portable-fetch';
declare module 'url';

/**
 * @export
 * @namespace Goal
 */
export namespace Goal {
    /**
     * @export
     * @enum {string}
     */
    export enum GoalSideEnum {
        Above = <any> 'above',
        Below = <any> 'below'
    }
}

/**
 * @export
 * @namespace GoalNewPayload
 */
export namespace GoalNewPayload {
    /**
     * @export
     * @enum {string}
     */
    export enum GoalSideEnum {
        Above = <any> 'above',
        Below = <any> 'below'
    }
}

/**
 * @export
 * @namespace TimeInterval
 */
export namespace TimeInterval {
    /**
     * @export
     * @enum {string}
     */
    export enum TypEnum {
        Monthly = <any> 'monthly',
        Annually = <any> 'annually',
        Daily = <any> 'daily',
        Weekly = <any> 'weekly'
    }
}

/**
 * @export
 * @namespace ValidateUserPayload
 */
export namespace ValidateUserPayload {
    /**
     * @export
     * @enum {string}
     */
    export enum TypEnum {
        Email = <any> 'email',
        Password = <any> 'password'
    }
}
