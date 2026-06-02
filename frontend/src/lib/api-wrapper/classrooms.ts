/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPost } from '$lib/api-wrapper/index';
import type { Option } from '$lib/api-wrapper/types';

const prefix = 'classrooms/';

function apiCallPostClassroom(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

function apiCallPostLesson(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

export type ClassroomId = number;
export type LessonId = number;

export type ClassroomAddData = {
    name: string;
    description: Option<string>;
};

export type ClassroomGeneratedData = {
    id: ClassroomId;
    joinCode: string;
};

export type LessonAddData = {
    name: string;
    description: Option<string>;
    published: Option<boolean>;
};

/** Add a classroom to the database */
export function addClassroom(classroomAddData: ClassroomAddData): Promise<ClassroomGeneratedData> {
    return apiCallPostClassroom('add', classroomAddData);
}

/** Add a lesson to a classroom in the database */
export function addLesson(
    lessonAddData: LessonAddData,
    classroomId: ClassroomId
): Promise<LessonId> {
    return apiCallPostLesson(`${classroomId}/add-lesson`, lessonAddData);
}
