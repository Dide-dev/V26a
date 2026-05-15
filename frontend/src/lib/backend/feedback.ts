/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPost, apiCallGet } from './index';
import type { PaginationOptions } from '$lib/pagination';

const prefix = 'feedback/';

function apiCallPostFeedback(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

function apiCallGetFeedback(path: string) {
    return apiCallGet(prefix + path);
}

/** The data corresponding to given feedback */
export type FeedbackData = {
    email: string;
    feedback: string;
};

/** The data returned when retrieving feedback. */
export type FeedbackListResponse = {
    feedbackTotal: number;
    feedback: FeedbackGetData[];
};

/** Data corresponding to a returned piece of feedback */
export type FeedbackGetData = {
    feedbackId: number;
    email: string;
    feedback: string;
    createdAt: Date;
};

/** Add feedback to the database */
export function provideFeedback(feedback: FeedbackData) {
    return apiCallPostFeedback('provide', feedback);
}

export function getPaginationParams(pageOptions: PaginationOptions): string {
    const params = new URLSearchParams();

    Object.entries(pageOptions).forEach(([key, value]) => {
        if (value !== null) {
            params.append(key, String(value));
        }
    });

    return String(params);
}

/** Get feedback from the database */
export function getFeedback(pageOptions: PaginationOptions): Promise<FeedbackListResponse> {
    const params: string = getPaginationParams(pageOptions);
    return apiCallGetFeedback(`get?${params}`);
}
