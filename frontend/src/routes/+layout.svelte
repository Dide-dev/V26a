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

    let { children } = $props();

    let errors: Error[] = $state([]);

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

    const leftLinks: Link[] = [
        {
            href: '/',
            text: m.home(),
            formatting: ''
        },
        {
            href: '/tree-generator/',
            text: m.createTree(),
            formatting: ''
        }
    ];
    const rightLinks: Link[] = [
        {
            href: '/registration/',
            text: m.register(),
            formatting: 'thin'
        },
        {
            href: '/login/',
            text: m.login(),
            formatting: 'thin'
        }
    ];
    const footerLinks: Link[] = [];
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<Navigation {leftLinks} {rightLinks}></Navigation>

{@render children()}

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
