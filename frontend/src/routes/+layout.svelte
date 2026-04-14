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
    let menu = $state(false);
    let userInfo = $state(getUserInfo());
    let pathname = $state(page.url.pathname);

    function openMenu() {
        menu = !menu;
    }

    function closeMenu() {
        menu = false;
    }

    // Retrieve userinfo & path after navigating to another page (refreshes navbar).
    afterNavigate(() => {
        const newUserInfo = getUserInfo();
        if (JSON.stringify(newUserInfo) !== JSON.stringify(userInfo)) {
            userInfo = newUserInfo;
            window.location.reload();
        }
        pathname = page.url.pathname;
    });
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<nav class="m-1 bg-sky-400 p-1">
    <div class="mb-0.5 flex justify-end">
        <button
            class="rounded-sm bg-sky-600 p-1 md:hidden"
            onclick={openMenu}
            aria-label="Toggle menu"
        >
            {#if menu}✕{:else}☰{/if}
        </button>
    </div>

    <div
        class="
        flex-col gap-2 *:rounded-sm *:bg-sky-600 *:p-1
        {menu ? 'flex' : 'hidden'}
        md:flex md:flex-row md:justify-end
    "
    >
        <a href={appLink('/')} onclick={closeMenu}>{m.home()}</a>

        {#if userInfo?.accountType === 'author' || userInfo?.accountType === 'admin'}
            <a href={appLink('/my-trees')} onclick={closeMenu}>{m.myTrees()}</a>
            <a href={appLink('/profile')} onclick={closeMenu}>{m.profile()}</a>
            <a href={appLink('/tree-generator')} onclick={closeMenu}>{m.createTree()}</a>
        {:else}
            <a href={appLink('/registration')} onclick={closeMenu}>{m.register()}</a>
            <a href={appLink('/login')} onclick={closeMenu}>{m.login()}</a>
        {/if}

        <!-- Locale is filtered to show 1 language at the same time-->
        {#each locales.filter((l) => l !== (pathname.startsWith('/en') ? 'en' : 'nl')) as locale (locale)}
            <!-- The 'data-sveltekit-reload' is important to let the entire page reload on locale change -->
            <a href={localizeHref(page.url.href, { locale })} data-sveltekit-reload>
                {locale}
            </a>
        {/each}
    </div>
</nav>

{@render children()}

<footer><p class="fixed right-1 bottom-1.5">© Utrecht University (ICS).</p></footer>
