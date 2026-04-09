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
    let menuOpen = $state(false);
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<nav class="m-1 bg-sky-400 p-1">
    <div class="flex justify-end">
        <button
            class="rounded-sm bg-sky-600 p-1 md:hidden"
            onclick={() => menuOpen = !menuOpen}
            aria-label="Toggle menu"
        >
            {#if menuOpen}✕{:else}☰{/if}
        </button>
    </div>

    <div class="
        flex-col gap-2 *:rounded-sm *:bg-sky-600 *:p-1
        {menuOpen ? 'flex' : 'hidden'}
        md:flex md:flex-row md:justify-end
    ">
        <a href={appLink('/')}>{m.home()}</a>
        <a href={appLink('/tree-generator')}>{m.createTree()}</a>
        <a href={appLink('/registration')}>{m.register()}</a>
        <a href={appLink('/login')}>{m.login()}</a>
        {#each locales as locale (locale)}
            <!-- The 'data-sveltekit-reload' is important to let the entire page reload on locale change -->
            <a href={localizeHref(page.url.href, { locale })} data-sveltekit-reload>
                {locale}
            </a>
        {/each}
    </div>
</nav>

{@render children()}

<footer><p class="fixed right-1 bottom-1.5">© Utrecht University (ICS).</p></footer>
