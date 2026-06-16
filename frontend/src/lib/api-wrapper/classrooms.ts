/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPost } from '$lib/api-wrapper/index';
import type { UserId } from './users';
import type { LessonId } from '$lib/api-wrapper/lessons';
import type { Option } from '$lib/api-wrapper/types';

const prefix = 'classrooms';

function apiCallPostClassroom(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

export type ClassroomId = number;

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

export type ClassroomUserInfo = {
    id: ClassroomId;
    num: number;
    name: string;
}

/** The data in the classroom cookie */
export type AllClassroomUserInfos = {
    claims: ClassroomUserInfo[];
};

/** Add a classroom to the database */
export function addClassroom(classroomAddData: ClassroomData): Promise<ClassroomGeneratedData> {
    return apiCallPostClassroom('', classroomAddData);
}

/** Add a lesson to a classroom in the database */
export function addLesson(
    lessonAddData: LessonAddData,
    classroomId: ClassroomId
): Promise<LessonId> {
    return apiCallPostClassroom(`/${classroomId}/lessons`, lessonAddData);
}

/** Log in to a classroom, creates a new user or logs in an existing user */
export function classroomLogin(
    classroomUserLoginData: ClassroomUserLoginData
): Promise<ClassroomLoginResponse> {
    return apiCallPostClassroom('/login', classroomUserLoginData);
}

/** Gets all information from the classroom_user_infos cookie */
export function getAllClassroomUserInfos(): AllClassroomUserInfos | null {
    if (typeof document === 'undefined') return null;
    const cookie = document.cookie
        .split('; ')
        .find((row) => row.startsWith('classroom_user_infos='));
    if (!cookie) return null;
    try {
        return JSON.parse(decodeURIComponent(cookie.split('=')[1]));
    } catch {
        return null;
    }
}

/** Gets information for one classroom from the classroom_user_infos cookie */
export function getClassroomUserInfo(classroomId: ClassroomId): ClassroomUserInfo | null {
    if (typeof document === 'undefined') return null;
    const cookie = document.cookie
        .split('; ')
        .find((row) => row.startsWith('classroom_user_infos='));
    if (!cookie) return null;
    try {
        const allClassroomUserInfos: AllClassroomUserInfos = JSON.parse(decodeURIComponent(cookie.split('=')[1]));
        allClassroomUserInfos.claims.forEach(classroomUserInfo => {
            if (classroomUserInfo.id == classroomId)
                return classroomUserInfo;
        })
        return null;
    } catch {
        return null;
    }
}
