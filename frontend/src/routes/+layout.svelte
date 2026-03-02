<script lang="ts">
    import { page } from '$app/state';
    import { appLink } from '$lib';
    import { locales, localizeHref } from '$lib/paraglide/runtime';
    import './layout.css';
    import favicon from '$lib/assets/favicon.svg';

    let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<nav
    class="m-1 flex justify-end gap-2 bg-sky-400 p-1 *:rounded-sm *:border-zinc-900 *:bg-sky-600 *:p-1"
>
    <a href={appLink('/')}>Home</a>
    <a href={appLink('/api-testing')}>Api Testing</a>
    <a href={appLink('/counter')}>Global Counter</a>
    <a href={appLink('/tree-viewer')}>View Example Tree</a>
    <a href={appLink('/tree-viewer')}>Add Card</a>
    {#each locales as locale (locale)}
        <!-- The 'data-sveltekit-reload' is important to let the entire page reload on locale change -->
        <a href={localizeHref(page.url.pathname, { locale })} data-sveltekit-reload>
            {locale}
        </a>
    {/each}
</nav>

{@render children()}
