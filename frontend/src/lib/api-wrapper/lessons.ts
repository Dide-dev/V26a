/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallEmptyPost } from '$lib/api-wrapper/index';
import type { TreeId } from './trees';

const prefix = 'lessons';

function apiCallPostTreeToLesson(path: string) {
    return apiCallEmptyPost(prefix + path);
}

export type LessonId = number;

/** Add an existing tree to a lesson in the database */
export function addTreeToLesson(lessonId: LessonId, treeId: TreeId): Promise<void> {
    return apiCallPostTreeToLesson(`/${lessonId}/trees/${treeId}`);
}
