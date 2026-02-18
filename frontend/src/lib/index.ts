// place files you want to import through the `$lib` alias in this folder.
import { localizeHref } from '$lib/paraglide/runtime';

// Must be used in any linking to other pages, like the 'a' html element.
// Localized and could possibly add prefix to the path.
export function appLink(href: string) {
	// TODO: Should 'resolve' from $app/paths be used??
	return localizeHref(href);
}
