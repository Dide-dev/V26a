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
    import { getUserInfo } from '$lib/backend/users';
    import { afterNavigate } from '$app/navigation';

    let { children } = $props();
    let menuOpen = $state(false);
    let userInfo = $state(getUserInfo());
    let pathname = $state(page.url.pathname);

    afterNavigate(() => {
        userInfo = getUserInfo();
        pathname = page.url.pathname;
    });

    function isActive(path: string) {
        const normalize = (p: string) => p.replace(/\/$/, '') || '/';
        return normalize(pathname) === normalize(localizeHref(path));
    }
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<nav class="m-1 bg-sky-400 p-1">
    <div class="flex justify-end">
        <button
            class="rounded-sm bg-sky-600 p-1 md:hidden"
            onclick={() => (menuOpen = !menuOpen)}
            aria-label="Toggle menu"
        >
            {#if menuOpen}✕{:else}☰{/if}
        </button>
    </div>

    <div
        class="
        flex-col gap-2 *:rounded-sm *:bg-sky-600 *:p-1
        {menuOpen ? 'flex' : 'hidden'}
        md:flex md:flex-row md:justify-end
    "
    >
        {#if !isActive('/')}
            <a href={appLink('/')} onclick={() => (menuOpen = false)}>{m.home()}</a>
        {/if}

        {#if isActive('/')}
            <a href={appLink('/')} onclick={() => (menuOpen = false)}>{m.trees()}</a>
            <a href={appLink('/')} onclick={() => (menuOpen = false)}>{m.credits()}</a>
        {/if}

        {#if userInfo?.accountType === 'author'}
            {#if isActive('/my-trees')}
                <a href={appLink('/tree-generator')} onclick={() => (menuOpen = false)}
                    >{m.createTree()}</a
                >
            {/if}
            {#if !isActive('/my-trees')}
                <a href={appLink('/my-trees')} onclick={() => (menuOpen = false)}>{m.myTrees()}</a>
            {/if}
            {#if !isActive('/profile')}
                <a href={appLink('/profile')} onclick={() => (menuOpen = false)}>{m.profile()}</a>
            {/if}
        {:else}
            {#if !isActive('/registration')}
                <a href={appLink('/registration')} onclick={() => (menuOpen = false)}
                    >{m.register()}</a
                >
            {/if}
            {#if !isActive('/login')}
                <a href={appLink('/login')} onclick={() => (menuOpen = false)}>{m.login()}</a>
            {/if}
        {/if}

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
