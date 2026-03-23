/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { beforeEach } from 'vitest';

beforeEach(() => {
    // Reset the DOM. This is necessary to ensure that each individual test starts
    // with an empty DOM, so that they do not interfere with each other.
    document.body.innerHTML = '';
});
