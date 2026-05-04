/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

// place files you want to import through the `$lib` alias in this folder.
import { getLocale, localizeHref } from '$lib/paraglide/runtime';

// Must be used in any linking to other pages, like the 'a' html element.
// Localized and could possibly add prefix to the path.
export function appLink(href: string) {
    // TODO: Should 'resolve' from $app/paths be used??
    return localizeHref(href);
}

const localeToTimeFormatString = { nl: 'nl-Nl', en: 'en-GB' };

// Used to format date and time.
// Note we will not support American time notation.
export const timeFormatString = localeToTimeFormatString[getLocale()];
