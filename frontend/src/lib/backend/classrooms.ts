/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPost } from '$lib/backend/index';

const prefix = '/classrooms';

function apiCallPostClassroom(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

export type ClassroomId = number;

export type ClassroomAddData = {
    name: string;
    description: string | null;
};

export type ClassroomGeneratedData = {
    id: ClassroomId;
    joinCode: string;
};

/** Add a classroom to the database */
export function addClassroom(classroomAddData: ClassroomAddData): Promise<ClassroomGeneratedData> {
    return apiCallPostClassroom('add', classroomAddData);
}
