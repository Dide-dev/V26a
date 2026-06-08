/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPost, apiCallGet } from '$lib/api-wrapper/index';
import { getPaginationParams, type PaginationOptions } from '$lib/pagination';
import type { UserId } from './users';
import type { Option } from '$lib/api-wrapper/types';

const prefix = 'classrooms';

function apiCallPostClassroom(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

function apiCallGetClassroom(path: string) {
    return apiCallGet(prefix + path);
}

export type ClassroomId = number;
export type LessonId = number;

export type ClassroomData = {
    name: string;
    description: string;
};

export type ClassroomGeneratedData = {
    id: ClassroomId;
    joinCode: string;
    createdAt: Date;
    ownerId: UserId;
};

export type ClassroomMetaData = {
    data: ClassroomData;
    genData: ClassroomGeneratedData;
};

export type ClassroomListResponse = {
    totalClassrooms: number;
    classrooms: ClassroomMetaData[];
};

export type LessonAddData = {
    name: string;
    description: Option<string>;
    published: Option<boolean>;
};

/** Add a classroom to the database */
export function addClassroom(classroomAddData: ClassroomData): Promise<ClassroomGeneratedData> {
    return apiCallPostClassroom('', classroomAddData);
}

/** Gets all the classrooms of a user from the database */
export function getAllOwnedClassrooms(
    userId: UserId,
    pageOptions: PaginationOptions
): Promise<ClassroomListResponse> {
    const params: string = getPaginationParams(pageOptions);
    return apiCallGetClassroom(`/users/${userId}/classrooms?${params}`);
}

/** Add a lesson to a classroom in the database */
export function addLesson(
    lessonAddData: LessonAddData,
    classroomId: ClassroomId
): Promise<LessonId> {
    return apiCallPostClassroom(`${classroomId}/lessons`, lessonAddData);
}
