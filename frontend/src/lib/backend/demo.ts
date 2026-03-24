/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallGet, apiCallPost } from './index';

type JsonTestStruct = {
    a: string;
    /** Current value of the global counter */
    b: number;
};

export function protectedTest(): Promise<JsonTestStruct> {
    return apiCallGet('demo/protected');
}

export function getJsonTest(): Promise<JsonTestStruct> {
    return apiCallGet('demo/json');
}

/** Increases the counter by 1 */
export function counterIncrement(): Promise<void> {
    return apiCallGet('demo/increment');
}

export type AddCounterBody = {
    add: number;
};

/** Increases the counter by arbitrary amount */
export function counterAdd(body: AddCounterBody): Promise<void> {
    return apiCallPost('demo/add-to-counter', body);
}
