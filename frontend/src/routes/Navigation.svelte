<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { page } from '$app/state';
    import { getLocale, locales, localizeHref } from '$lib/paraglide/runtime';
    import { appLink } from '$lib';
    import { MediaQuery } from 'svelte/reactivity';

    let {
        leftLinks,
        rightLinks
    }: {
        leftLinks: NavLink[];
        rightLinks: NavLink[];
    } = $props();

    export type NavLink = {
        href: string;
        text: string;
        isThin: boolean;
    };

    let menu = $state(false);

    const windowIsLarge = new MediaQuery('min-width: 48rem');

    function toggleMenu() {
        menu = !menu;
    }

    function closeMenu() {
        menu = false;
    }
</script>

{#snippet navItem(link: NavLink)}
    <a
        class={[link.isThin && 'min-w-nav-thin', 'nav-item not-md:hover:bg-tree-450']}
        href={appLink(link.href)}
        onclick={closeMenu}
    >
        {link.text}
    </a>
{/snippet}

<nav class="w-100vw mb-6 flex font-bold text-neutral-100 not-md:flex-col not-md:justify-end">
    {#if !windowIsLarge.current}
        <button
            class={[
                menu && 'mb-0.5',
                'flex justify-end hover:bg-tree-500 hover:text-white md:w-screen'
            ]}
            onclick={toggleMenu}
            aria-label="Toggle menu"
        >
            {#if menu}✕{:else}☰{/if}
        </button>
    {/if}
    {#if menu || windowIsLarge.current}
        {#each leftLinks as link (link)}
            {@render navItem(link)}
        {/each}
        {#if windowIsLarge.current}
            <div class="min-w-0 flex-auto p-0"></div>
        {/if}
        {#each rightLinks as link (link)}
            {@render navItem(link)}
        {/each}
        <div class="nav-item dropdown min-w-nav-extra-thin">
            <div class="dropbutton not-md:hover:bg-tree-500">{getLocale()}</div>
            <div class="dropcontent">
                {#each locales.filter((l) => l !== getLocale()) as locale (locale)}
                    <a
                        class="not-md:w-screen md:min-w-nav-extra-thin"
                        href={localizeHref(page.url.href, { locale })}
                        data-sveltekit-reload
                        onclick={closeMenu}
                    >
                        {locale}
                    </a>
                {/each}
            </div>
        </div>
    {/if}
</nav>

<style>
    @layer components {
        nav > * {
            min-width: var(--spacing-nav-normal);

            background-color: var(--color-tree-625);

            @media (width >= 48rem) {
                --tw-gradient-position: to right;
                @supports (background-image: linear-gradient(in lab, red, red)) {
                    --tw-gradient-position: to right in oklab;
                }
                background-image: linear-gradient(var(--tw-gradient-stops));
                --tw-gradient-to: var(--color-tree-625);
                --tw-gradient-stops: var(--tw-gradient-via-stops);
                --tw-gradient-via: var(--color-tree-600);
                --tw-gradient-via-stops:
                    var(--tw-gradient-position), var(--tw-gradient-from), var(--tw-gradient-via),
                    var(--tw-gradient-to);
                --tw-gradient-from: var(--color-tree-600);
            }

            padding: 0.5rem 1rem;

            text-align: center;
        }

        nav > .nav-item:hover {
            --tw-gradient-from: var(--color-tree-500);
            --tw-gradient-via: var(--color-tree-500);
            --tw-gradient-to: var(--color-tree-500);

            color: var(--color-white);
        }

        nav > .nav-item:hover + * {
            --tw-gradient-from: var(--color-tree-625);
        }

        .dropdown {
            padding: 0;
        }

        .dropdown .dropbutton {
            padding: 0.5rem 1rem;
        }

        .dropcontent {
            display: none;
            background-color: var(--color-tree-150);
            color: var(--color-neutral-600);
        }

        .dropcontent:hover {
            background-color: var(--color-tree-300);
            color: var(--color-black);
        }

        .dropdown .dropcontent > * {
            padding: 0.5rem 1rem;
            display: block;
        }

        .dropdown:hover .dropcontent {
            position: absolute;
            display: block;
        }
    }
</style>
