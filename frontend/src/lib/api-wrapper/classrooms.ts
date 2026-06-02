/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPost, apiCallGet } from '$lib/api-wrapper/index';
import type { Option } from '$lib/api-wrapper/types';
import type { UserId } from './users';

const prefix = 'classrooms/';

function apiCallPostClassroom(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

function apiCallGetClassroom(path: string) {
    return apiCallGet(prefix + path);
}

export type ClassroomId = number;

export type ClassroomAddData = {
    name: string;
    description: Option<string>;
};

export type ClassroomAddReturnData = {
    id: ClassroomId;
    joinCode: string;
};

export type ClassroomGeneratedData = {
    id: ClassroomId;
    joinCode: string;
    createdAt: Date;
    ownerId: UserId;
};

export type ClassroomAllData = {
    data: ClassroomAddData;
    genData: ClassroomGeneratedData;
};

/** Add a classroom to the database */
export function addClassroom(classroomAddData: ClassroomAddData): Promise<ClassroomAddReturnData> {
    return apiCallPostClassroom('add', classroomAddData);
}

export function getAllOwnedClassrooms(id: UserId): Promise<ClassroomAllData[]> {
    return apiCallGetClassroom(`classroom/get-by-user/${id}`);
}
