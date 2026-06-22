/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import type { ClassroomId } from '$lib/api-wrapper/classrooms';
import type { AccountType, UserId, Username } from '$lib/api-wrapper/users';

/** The data in the cookie */
export type UserInfo = {
    name: Username;
    nickname: string;
    /** This corresponds to userid of the current login. */
    userId: UserId;
    accountType: AccountType;
};

export type ClassroomUserInfo = {
    id: ClassroomId;
    num: number;
    name: string;
};

/** The data in the classroom cookie */
export type AllClassroomUserInfos = {
    sessions: ClassroomUserInfo[];
};

/**
 * Read the userinfo from the cookie.
 * If logged in return id & type and if not logged in return null.
 */
export function getUserInfo(): UserInfo | null {
    if (typeof document === 'undefined') return null;
    const cookie = document.cookie.split('; ').find((row) => row.startsWith('user_info='));
    if (!cookie) return null;
    try {
        return JSON.parse(decodeURIComponent(cookie.split('=')[1]));
    } catch {
        return null;
    }
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
        const allClassroomUserInfos: AllClassroomUserInfos = JSON.parse(
            decodeURIComponent(cookie.split('=')[1])
        );
        for (const classroomUserInfo of allClassroomUserInfos.sessions) {
            if (classroomUserInfo.id == classroomId) return classroomUserInfo;
        }
        return null;
    } catch {
        return null;
    }
}
