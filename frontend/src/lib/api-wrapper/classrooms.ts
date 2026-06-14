/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallGet, apiCallPost } from '$lib/api-wrapper/index';
import type { UserId } from './users';
import type { LessonId } from '$lib/api-wrapper/lessons';
import type { Option } from '$lib/api-wrapper/types';
import type { TreeMetadata } from '$lib/api-wrapper/trees';

const prefix = 'classrooms';

function apiCallPostClassroom(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

function apiCallGetClassroom(path: string) {
    return apiCallGet(prefix + path);
}

export type ClassroomId = number;

export type ClassroomAddData = {
    name: string;
    description: string;
};

export type ClassroomGeneratedData = {
    id: ClassroomId;
    joinCode: string;
}

export type ClassroomMetaData = {
    id: ClassroomId;
    name: string;
    description: string;
    createdAt: Date;
    ownerId: UserId;
};

export type ClassroomListResponse = {
    totalClassrooms: number;
    classrooms: ClassroomMetaData[];
};

export type ClassroomUserLoginData = {
    joinCode: string;
    username: string;
};

export type ClassroomLoginResponse =
    | ClassroomLoginResponseSuccess
    | ClassroomLoginResponseJoinCodeIncorrect
    | ClassroomLoginResponseMissingUser;
export type ClassroomLoginResponseSuccess = {
    type: 'Success';
    id: ClassroomId;
    num: number;
    name: string;
};
export type ClassroomLoginResponseJoinCodeIncorrect = {
    type: 'JoinCodeIncorrect';
    joinCode: string;
};
export type ClassroomLoginResponseMissingUser = {
    type: 'MissingUser';
    userNumber: number;
};

export type ClassroomTeacherDashboardData = {
    metadata: ClassroomMetaData;
    lessons: LessonWithTrees[];
    joinCode: string;
}

export type ClassroomStudentDashboardData = {
    metadata: ClassroomMetaData;
    lessons: LessonWithTrees[];
}

export type LessonAddData = {
    name: string;
    description: Option<string>;
    published: Option<boolean>;
};

export type LessonMetaData = {
    id: LessonId;
    orderNumber: number;
    createdAt: Date;
    name: string;
    description: string;
    published: boolean;
}

export type LessonWithTrees = {
    metadata: LessonMetaData;
    trees: TreeMetadata[];
}

/** Add a classroom to the database */
export function addClassroom(classroomAddData: ClassroomAddData): Promise<ClassroomGeneratedData> {
    return apiCallPostClassroom('', classroomAddData);
}

/** Log in to a classroom, creates a new user or logs in an existing user */
export function classroomLogin(
    classroomUserLoginData: ClassroomUserLoginData
): Promise<ClassroomLoginResponse> {
    return apiCallPostClassroom('login', classroomUserLoginData);
}

export function getClassroomTeacherDashboard(classroomId: ClassroomId): Promise<ClassroomTeacherDashboardData> {
    return apiCallGetClassroom(`/${classroomId}/teacher-dash`);
}

export function getClassroomStudentDashboard(classroomId: ClassroomId): Promise<ClassroomStudentDashboardData> {
    return apiCallGetClassroom(`/${classroomId}/student-dash`);
}

/** Add a lesson to a classroom in the database */
export function addLesson(
    lessonAddData: LessonAddData,
    classroomId: ClassroomId
): Promise<LessonId> {
    return apiCallPostClassroom(`/${classroomId}/lessons`, lessonAddData);
}
