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
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<nav
    class="m-1 flex justify-end gap-2 bg-sky-400 p-1 *:rounded-sm *:border-zinc-900 *:bg-sky-600 *:p-1"
>
    <a href={appLink('/')}>{m.home()}</a>
    <a href={appLink('/api-testing')}>{m.apiTesting()}</a>
    <a href={appLink('/counter')}>{m.globalCounter()}</a>
    <a href={appLink('/tree-viewer')}>{m.viewExampleTree()}</a>
    <a href={appLink('/tree-generator')}>{m.createTree()}</a>
    <a href={appLink('/registration')}>{m.register()}</a>
    <a href={appLink('/login')}>{m.login()}</a>
    {#each locales as locale (locale)}
        <!-- The 'data-sveltekit-reload' is important to let the entire page reload on locale change -->
        <a href={localizeHref(page.url.pathname, { locale })} data-sveltekit-reload>
            {locale}
        </a>
    {/each}
</nav>

{@render children()}

<footer><p class="fixed right-1 bottom-1.5">© Utrecht University (ICS).</p></footer>
