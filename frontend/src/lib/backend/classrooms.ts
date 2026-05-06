/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPost } from '$lib/backend/index';

const prefix = '/classrooms';

function apiCallPostCard(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

export type ClassroomId = number;

export type ClassroomAddData = {
    name: string;
    description: string;
};

export type ClassroomCreatedData = {
    id: ClassroomId;
    joinCode: string;
};

/** Add a classroom to the database */
export function addClassroom(classroomAddData: ClassroomAddData): Promise<ClassroomCreatedData> {
    return apiCallPostCard('add', classroomAddData);
}
