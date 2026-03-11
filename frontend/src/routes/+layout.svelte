<script lang="ts">
    import { page } from '$app/state';
    import { appLink } from '$lib';
    import { locales, localizeHref } from '$lib/paraglide/runtime';
    import './layout.css';
    import favicon from '$lib/assets/favicon.svg';
    import { m } from '$lib/paraglide/messages.js';

    let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<nav
    class="m-1 flex justify-end gap-2 bg-sky-400 p-1 *:rounded-sm *:border-zinc-900 *:bg-sky-600 *:p-1"
>
    <a href={appLink('/')}>{m.Home()}</a>
    <a href={appLink('/api-testing')}>{m.ApiTesting()}</a>
    <a href={appLink('/counter')}>{m.GlobalCounter()}</a>
    <a href={appLink('/tree-viewer')}>{m.ViewExampleTree()}</a>
    <a href={appLink('/registration')}>{m.Register()}</a>
    <a href={appLink('/login')}>{m.Login()}</a>
    {#each locales as locale (locale)}
        <!-- The 'data-sveltekit-reload' is important to let the entire page reload on locale change -->
        <a href={localizeHref(page.url.pathname, { locale })} data-sveltekit-reload>
            {locale}
        </a>
    {/each}
</nav>

{@render children()}
