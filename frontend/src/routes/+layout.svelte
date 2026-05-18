<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import './layout.css';
    import favicon from '$lib/assets/favicon.svg';
    import { m } from '$lib/paraglide/messages.js';
    import Navigation from './Navigation.svelte';
    import { NavLink } from '$lib/navigation';
    import Footer from './Footer.svelte';
    import { FooterLink } from '$lib/footer';
    import { getUserInfo } from '$lib/backend/users';
    import { afterNavigate } from '$app/navigation';
    import { getLocale, locales, localizeHref } from '$lib/paraglide/runtime.js';
    import { page } from '$app/state';
    import { Tween } from 'svelte/motion';

    let { children } = $props();
    let userInfo = $state(getUserInfo());
    let errors: Error[] = $state([]);

    // Retrieve userinfo after navigating to another page (refreshes nav-bar).
    afterNavigate(() => {
        const newUserInfo = getUserInfo();
        if (JSON.stringify(newUserInfo) !== JSON.stringify(userInfo)) {
            userInfo = newUserInfo;
            window.location.reload();
        }
    });

    function handleWindowError(event: Event) {
        if (event instanceof ErrorEvent) {
            errors.push(event.error);
        } else {
            errors.push(new Error('Unknown window error.'));
        }
    }
    function handleError(err: PromiseRejectionEvent) {
        errors.push(err.reason);
    }

    let leftLinks: NavLink[] = $derived.by(() => {
        let result: NavLink[] = [
            new NavLink({
                defaultHref: '/',
                text: m.home()
            })
        ];

        if (userInfo === null) {
            result.push(
                new NavLink({
                    defaultHref: '/all-trees',
                    text: m.allTrees()
                })
            );
        } else {
            result.push(
                new NavLink({
                    defaultHref: '/all-trees',
                    text: m.trees(),
                    children: [
                        {
                            href: '/all-trees',
                            text: m.allTrees()
                        },
                        {
                            href: '/my-trees',
                            text: m.myTrees()
                        },
                        {
                            href: '/tree-generator',
                            text: m.createTree()
                        }
                    ]
                })
            );
            if (userInfo.accountType == 'admin') {
                result.push(
                    new NavLink({
                        defaultHref: '/user-list',
                        text: m.admin(),
                        children: [
                            {
                                href: '/user-list',
                                text: m.userList()
                            }
                        ]
                    })
                );
            }
        }

        return result;
    });
    let rightLinks: NavLink[] = $derived.by(() => {
        let result: NavLink[] = [];

        if (userInfo !== null) {
            result = [
                new NavLink({
                    defaultHref: '/profile',
                    text: m.account(),
                    props: {
                        width: 'th',
                        localeSwitch: false
                    },
                    children: [
                        {
                            href: '/profile',
                            text: m.profile()
                        },
                        {
                            href: '/login',
                            text: m.login()
                        }
                    ]
                })
            ];
        } else {
            result = [
                new NavLink({
                    defaultHref: '/login',
                    text: m.account(),
                    props: {
                        width: 'th',
                        localeSwitch: false
                    },
                    children: [
                        {
                            href: '/login',
                            text: m.login()
                        },
                        {
                            href: '/registration',
                            text: m.register()
                        }
                    ]
                })
            ];
        }

        result.push(
            new NavLink({
                defaultHref: page.url.href,
                text: getLocale(),
                props: {
                    width: 'xt',
                    localeSwitch: true
                },
                children: locales
                    .filter((locale) => locale !== getLocale())
                    .map((locale) => {
                        return {
                            href: localizeHref(page.url.href, { locale }),
                            text: locale
                        };
                    })
            })
        );

        return result;
    });

    const footerLinks: FooterLink[] = [
        new FooterLink({
            href: '/privacy',
            text: m.privacy()
        }),
        new FooterLink({
            href: '/colophon',
            text: m.colophon()
        }),
        new FooterLink({
            href: '/feedback',
            text: m.feedback()
        }),
        new FooterLink({
            href: '/api-docs/',
            text: m.apiDocs(),
            target: '_blank',
            localized: false
        }),
        new FooterLink({
            href: 'https://softwareprojecten.sites.uu.nl/',
            text: m.otherProjects(),
            target: '_blank',
            localized: false
        })
    ];

    let offsets: Tween<number>[] = $state([]);
    let offset = $derived(offsets.reduce((acc, curr) => acc + curr.current, 0));
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<Navigation {leftLinks} {rightLinks} bind:offsets />

<main style="margin-top: {offset}px;" class="flex-auto">
    {@render children()}
</main>

<svelte:window onerror={handleWindowError} onunhandledrejection={handleError} />

{#if errors.length > 0}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
        {#each errors as err (err)}
            <div class="flex flex-col gap-2 rounded bg-neutral-500 p-4 text-red-800">
                <b>{m.unexpectedErrorOccurred()}</b>
                <b>{err.message}</b>
                <button
                    class="rounded bg-blue-800 px-3 py-1 text-white"
                    onclick={() => errors.shift()}>{m.close()}</button
                >
            </div>
        {/each}
    </div>
{/if}

<Footer links={footerLinks} />

<!-- Necessary to have svelte-kit load in the pages of the other locales. -->
<div class="hidden">
    {#each locales as locale (locale)}
        <a href={localizeHref(page.url.href, { locale })}>{locale}</a>
    {/each}
</div>
