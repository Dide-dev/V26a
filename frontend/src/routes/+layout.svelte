<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import './layout.css';
    import favicon from '$lib/assets/favicon.svg';
    import { m } from '$lib/paraglide/messages.js';
    import Navigation, { type Link } from './Navigation.svelte';
    import Footer from './Footer.svelte';
    import { getUserInfo } from '$lib/backend/users';
    import { afterNavigate } from '$app/navigation';

    let { children } = $props();
    let userInfo = $state(getUserInfo());
    let errors: Error[] = $state([]);

    // Retrieve userinfo & path after navigating to another page (refreshes navbar).
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

    let leftLinks: Link[] = $derived.by(() => {
        let result = [
            {
                href: '/',
                text: m.home(),
                formatting: ''
            },
            {
                href: '/all-trees',
                text: m.allTrees(),
                formatting: ''
            }
        ];

        if (userInfo !== null) {
            result.push({
                href: '/tree-generator',
                text: m.createTree(),
                formatting: ''
            });
        }

        return result;
    });
    let rightLinks: Link[] = $derived.by(() => {
        let result = [];

        if (userInfo !== null) {
            result = [
                {
                    href: '/profile',
                    text: m.profile(),
                    formatting: 'thin'
                },
                {
                    href: '/my-trees',
                    text: m.myTrees(),
                    formatting: 'thin'
                }
            ];
        } else {
            result = [
                {
                    href: '/registration',
                    text: m.register(),
                    formatting: 'thin'
                },
                {
                    href: '/login',
                    text: m.login(),
                    formatting: 'thin'
                }
            ];
        }

        return result;
    });
    const footerLinks: Link[] = [
        {
            href: '/colophon',
            text: m.colophon(),
            formatting: ''
        },
        {
            href: '/',
            text: m.home(),
            formatting: ''
        }
    ];
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<Navigation {leftLinks} {rightLinks}></Navigation>

<main class="flex-auto">
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

<Footer links={footerLinks}></Footer>
