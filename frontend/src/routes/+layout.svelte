<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { page } from '$app/state';
    import { appLink } from '$lib';
    import { locales, localizeHref } from '$lib/paraglide/runtime';
    import './layout.css';
    import favicon from '$lib/assets/favicon.svg';
    import { m } from '$lib/paraglide/messages.js';

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

<nav
    class="m-1 flex justify-end gap-2 bg-sky-400 p-1 *:rounded-sm *:border-zinc-900 *:bg-sky-600 *:p-1"
>
    <a href={appLink('/')}>{m.home()}</a>
    <a href={appLink('/tree-generator')}>{m.createTree()}</a>
    <a href={appLink('/registration')}>{m.register()}</a>
    <a href={appLink('/login')}>{m.login()}</a>
    <a href={appLink('/all-trees')}>All trees</a>
    <a href={appLink('/my-trees')}>My trees</a>
    {#each locales as locale (locale)}
        <!-- The 'data-sveltekit-reload' is important to let the entire page reload on locale change -->
        <a href={localizeHref(page.url.href, { locale })} data-sveltekit-reload>
            {locale}
        </a>
    {/each}
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
