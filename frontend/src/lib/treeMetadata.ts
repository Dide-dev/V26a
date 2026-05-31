/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { m } from '$lib/paraglide/messages.js';
import { type Option } from './api-wrapper/types';
import { minInputLengths, maxInputLengths } from './api-wrapper/trees';

/**
 * Returns either the list of tags parsed into an array if the input is valid,
 * or an error message as a string if it is not.
 */
export function tryParseTags(tags: string): string[] | string {
    if (tags.trim() === '') {
        return [];
    }
    const list = tags.split(',').map((t) => t.trim());
    if (list.some((t) => t.length < minInputLengths.tag || t.length > maxInputLengths.tag)) {
        return m.tagLengthRequirements();
    }
    if (list.some((t) => /\s/.test(t))) {
        return m.noWhitespaceInTags();
    }
    if (list.some((t) => !/^[a-zA-Z0-9._-]+$/.test(t))) {
        return m.validTagCharacters();
    }
    return list;
}

/**
 * Returns either the normalized value of the ISBN if it is valid,
 * or `null` if it is not.
 */
export function tryParseISBN(isbn: Option<string>): Option<string> {
    if (!isbn) {
        return '';
    }

    isbn = isbn.trim().replaceAll('-', '');

    if (isbn.length === 10 && checkISBN10(isbn)) {
        return isbn;
    } else if (isbn.length === 13 && checkISBN13(isbn)) {
        return isbn;
    } else {
        return null;
    }
}

function checkISBN10(str: string): boolean {
    let s = 0,
        t = 0;

    for (let i = 0; i < 9; i++) {
        const d = parseInt(str[i]);

        if (isNaN(d)) {
            return false;
        }

        t += d;
        s += t;
    }

    s += t;

    const lastChar = str[9];
    if (lastChar === 'X') {
        // X is used to indicate a checksum digit value of '10'
        return 11 - (s % 11) === 10;
    } else {
        return (11 - (s % 11)) % 11 === parseInt(lastChar);
    }
}

function checkISBN13(str: string): boolean {
    let w = 1,
        s = 0;

    for (const c of str) {
        const d = parseInt(c);

        if (isNaN(d)) {
            return false;
        }

        s += w * d;
        w = (w + 2) % 4;
    }

    return s % 10 === 0;
}
