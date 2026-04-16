/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPost } from './index';
import type { Option } from './types';

const prefix = 'feedback/';

function apiCallPostFeedback(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

/** The data corresponding to given feedback */
export type FeedbackData = {
    email: Option<string>;
    positive: Option<string>;
    negative: Option<string>;
    additional: Option<string>;
    grade: Option<number>;
};

/** Add feedback to the database */
export function provideFeedback(feedback: FeedbackData) {
    return apiCallPostFeedback('provide', feedback);
}
