import { m } from '$lib/paraglide/messages.js';

export function booleanToString(value: boolean) {
    if (value) return m.yes();
    return m.no();
}
