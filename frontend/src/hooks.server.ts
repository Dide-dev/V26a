/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import type { Handle, fail, redirect } from '@sveltejs/kit';
import { paraglideMiddleware } from '$lib/paraglide/server';

const handleParaglide: Handle = ({ event, resolve }) =>
    paraglideMiddleware(event.request, ({ request, locale }) => {
        event.request = request;

        return resolve(event, {
            transformPageChunk: ({ html }) => html.replace('%paraglide.lang%', locale)
        });
    });

export const handle: Handle = handleParaglide;
