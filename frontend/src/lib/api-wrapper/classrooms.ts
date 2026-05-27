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

export type ClassroomId = number;

export type ClassroomAddData = {
    name: string;
    description: Option<string>;
};

export type ClassroomGeneratedData = {
    id: ClassroomId;
    joinCode: string;
};

export type ClassroomUserLoginData = {
    joinCode: string;
    username: string;
};

export type ClassroomLoginResponse = {
    id: ClassroomId;
    num: number;
};

/** Add a classroom to the database */
export function addClassroom(classroomAddData: ClassroomAddData): Promise<ClassroomGeneratedData> {
    return apiCallPostClassroom('add', classroomAddData);
}

/** Log in to a classroom, creates a new user or logs in an existing user */
export function classroomLogin(
    classroomUserLoginData: ClassroomUserLoginData
): Promise<ClassroomLoginResponse> {
    return apiCallPostClassroom('login', classroomUserLoginData);
}
