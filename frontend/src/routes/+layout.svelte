<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { page } from '$app/state';
    import { appLink } from '$lib';
    import { getLocale, locales, localizeHref } from '$lib/paraglide/runtime';
    import './layout.css';
    import favicon from '$lib/assets/favicon.svg';
    import { m } from '$lib/paraglide/messages.js';
    import LanguageDropdown from './LanguageDropdown.svelte';
    import { url } from 'inspector';

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
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<nav class="flex mb-1 *:text-center *:via-tree-600 *:min-w-50 *:p-2 *:pr-4 *:pl-4 *:bg-linear-to-r *:from-tree-600 *:to-tree-625 text-neutral-100 *:hover:text-white [&>a]:hover:from-tree-450 [&>a]:hover:via-tree-450 [&>a]:hover:to-tree-450 w-100vw justify-between ">
    <a class="peer/1" href={appLink('/')}>{m.home()}</a>
    <a class="peer/2 peer-hover/1:from-tree-625" href={appLink('/tree-generator')}>{m.createTree()}</a>
    <div class="flex-auto peer-hover/2:from-tree-625"></div>
    <a class="peer/3" href={appLink('/registration')}>{m.register()}</a>
    <a class="peer/4 peer-hover/3:from-tree-625"href={appLink('/login')}>{m.login()}</a>
    <div class="peer/parent peer-hover/4:from-tree-625">
        <a href = {localizeHref(page.url.href)}>{getLocale()}</a>
        {#each locales.filter((l) => l !== getLocale()) as locale (locale)}
            <a class="hidden peer-hover/parent:visible" href={localizeHref(page.url.href, { locale })} data-sveltekit-reload>
                {locale}
            </a>
        {/each}
    </div>
</nav>

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

<footer><p class="fixed right-1 bottom-1.5">© Utrecht University (ICS).</p></footer>
