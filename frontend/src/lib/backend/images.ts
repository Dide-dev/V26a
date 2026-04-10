/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

const prefix = 'images/';

async function apiCallAddImage(imageFile: File) {
    const response = await fetch(`/api/v1/${prefix}add`, {
        method: 'POST',
        body: imageFile
    });

    if (!response.ok) {
        const errorText = await response.text();
        throw new Error(errorText);
    }

    return response.text();
}

/** Add an image to the database. Returns the path on which the image can be found. */
export function uploadImage(imageFile: File): Promise<string> {
    return apiCallAddImage(imageFile);
}
