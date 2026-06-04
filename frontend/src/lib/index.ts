/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

// place files you want to import through the `$lib` alias in this folder.
import { getLocale, localizeHref } from '$lib/paraglide/runtime';
import { m } from '$lib/paraglide/messages.js';

// Must be used in any linking to other pages, like the 'a' html element.
// This localizes and possibly adds a prefix to the path.
export function appLink(href: string) {
    // We currently don't need to use `resolve` from $app/paths.
    // This is because we don't use dynamic routes with '/[some-path-argument]/'.
    // We are missing out on type checking which we could do through the `PathnameWithSearchOrHash` type,
    // but half of our links are in the header and footer where everything is just a `string`.

    return localizeHref(href);
}

const localeToTimeFormatString = { nl: 'nl-Nl', en: 'en-GB' };

// Used to format date and time.
// Note we will not support American time notation.
export const timeFormatString = localeToTimeFormatString[getLocale()];

export function booleanToString(value: boolean) {
    if (value) return m.yes();
    return m.no();
}
