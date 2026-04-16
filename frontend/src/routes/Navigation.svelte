<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { page } from '$app/state';
    import { getLocale, locales, localizeHref } from '$lib/paraglide/runtime';
    import { appLink } from '$lib';

    let {
        leftLinks,
        rightLinks
    }: {
        leftLinks: Link[];
        rightLinks: Link[];
    } = $props();

    export type Link = {
        href: string;
        text: string;
        formatting: string;
    };
</script>

<nav class="w-100vw mb-6 flex font-bold text-neutral-100">
    {#each leftLinks as link (link)}
        <a class="nav-item {link.formatting}" href={appLink(link.href)}>{link.text}</a>
    {/each}
    <div class="min-w-0 flex-auto p-0"></div>
    {#each rightLinks as link (link)}
        <a class="nav-item {link.formatting}" href={appLink(link.href)}>{link.text}</a>
    {/each}
    <div class="nav-item thin dropdown">
        <div class="dropbutton">{getLocale()}</div>
        <div class="dropcontent">
            {#each locales.filter((l) => l !== getLocale()) as locale (locale)}
                <a href={localizeHref(page.url.href, { locale })} data-sveltekit-reload>
                    {locale}
                </a>
            {/each}
        </div>
    </div>
</nav>

<style>
    @layer components {
        nav > * {
            min-width: 15vw;
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

            padding: 0.5rem 1rem;

            text-align: center;
        }

        nav > .thin {
            min-width: 6vw;
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
            margin: 0;
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

        .dropdown:hover .dropcontent > * {
            min-width: 6vw;
        }
    }
</style>
