/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPostFile } from './index';

const prefix = 'images/';

function apiCallPostImage(path: string, body: File) {
    return apiCallPostFile(prefix + path, body);
}

/** Add an image to the database. Returns the path on which the image can be found. */
export async function uploadImage(imageFile: File): Promise<string> {
    const path = await apiCallPostImage('add', imageFile);

    // This path should never be explored, but needs to be specified to remove undefined from the union type.
    if (path === undefined) {
        throw new Error('Image upload failed.');
    }

    return path;
}
