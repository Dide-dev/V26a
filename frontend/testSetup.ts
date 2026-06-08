/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { beforeEach } from 'vitest';
import * as ResizeObserverModule from 'resize-observer-polyfill';

window.matchMedia =
    window.matchMedia ||
    function (query: string) {
        return {
            matches: false,
            media: query,
            onchange: null,
            addListener: () => {},
            removeListener: () => {},
            addEventListener: () => {},
            removeEventListener: () => {},
            dispatchEvent: () => false
        } as MediaQueryList;
    };

beforeEach(() => {
    // Reset the DOM. This is necessary to ensure that each individual test starts
    // with an empty DOM, so that they do not interfere with each other.
    document.body.innerHTML = '';

    /* eslint-disable @typescript-eslint/no-explicit-any */
    (global as any).ResizeObserver = ResizeObserverModule.default;
});
