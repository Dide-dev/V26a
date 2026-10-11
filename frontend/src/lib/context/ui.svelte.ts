/** Global UI state to toggle full-screen. */

import { createContext } from 'svelte';

/** State of the top-level UI. Determines whether navigation bar and footer are displayed. */
export class UIState {
    visible = $state(true);

    /** Toggle the current UI state. */
    toggle() {
        this.visible = !this.visible;
    }

    /** Exit full-screen, revealing the nav-bar and footer. */
    minimize() {
        this.visible = true;
    }

    /** Enter full-screen, concealing the nav-bar and footer. */
    maximize() {
        this.visible = false;
    }
}

/** Global context getter and setter for the UI state. */
export const [getUIState, setUIState] = createContext<UIState>();
