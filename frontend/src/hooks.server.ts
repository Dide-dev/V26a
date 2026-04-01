/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import type { Handle, fail, redirect } from '@sveltejs/kit';
import { paraglideMiddleware } from '$lib/paraglide/server';
import { sequence } from '@sveltejs/kit/hooks';

const handleParaglide: Handle = ({ event, resolve }) =>
    paraglideMiddleware(event.request, ({ request, locale }) => {
        event.request = request;

        return resolve(event, {
            transformPageChunk: ({ html }) => html.replace('%paraglide.lang%', locale)
        });
    });

// creates a not-httpy only cookie with the userinfo
const handleSession: Handle = async ({ event, resolve }) => {
    const session = event.cookies.get('session');

    if (session) {
        try {
            const payload = JSON.parse(atob(session.split('.')[1]));
            event.cookies.set(
                'user_info',
                JSON.stringify({
                    userId: payload.sub,
                    accountType: payload.account_type
                }),
                {
                    path: '/',
                    httpOnly: false,
                    maxAge: payload.exp - Math.floor(Date.now() / 1000)
                }
            );
        } catch {
            event.cookies.delete('user_info', { path: '/' });
        }
    } else {
        event.cookies.delete('user_info', { path: '/' });
    }

    return resolve(event);
};

export const handle: Handle = sequence(handleSession, handleParaglide);
