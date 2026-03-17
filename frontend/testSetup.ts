import { beforeEach } from 'vitest';

beforeEach(() => {
    // Reset the DOM. This is necessary to ensure that each individual test starts
    // with an empty DOM, so that they do not interfere with each other.
    document.body.innerHTML = '';
});
