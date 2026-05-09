<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render a footer with the provided links.

    ## Reactivity
    The footer renders slightly differently depending on screen size:
    - **Larger windows (>= 48rem):** The logo, links and copyright notice get aligned horizontally.
      The links are capped at 4 per column, and spread out evenly over multiple columns if this
      number gets exceeded.
    - **Smaller windows (< 48rem):** The links are placed above the the logo and copyright notice.
      The links are placed in either 1 or 2 columns, and their length is uncapped.

    # Params
    - `links: FooterLink[]` - The links to render in the footer.
    - `offset: Tween<number>` - A `Tween` object with the current offset from the nav-bar.
-->
<script lang="ts">
    import { appLink } from '$lib';
    import { m } from '$lib/paraglide/messages.js';
    import logo from '$lib/assets/logo.png';
    import type { Tween } from 'svelte/motion';
    import { MediaQuery } from 'svelte/reactivity';
    import { FooterLink } from '$lib/footer';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';

    let {
        links,
        offset
    }: {
        /** The links to render in the footer. */
        links: FooterLink[];
        /** The current offset to render at. */
        offset: Tween<number>;
    } = $props();

    // 48rem === 768px
    /** Query whether the window is large or small. 'true' if small, and 'false' if not. */
    const windowIsSmall = new MediaQuery('not (min-width: 48rem)');
    /**
     * `links` divided over multiple separate arrays, to be displayed over separate columns.
     *
     * - In a large window, column length is capped by `maxColumnLengthLargeWindow`.
     * - In a small window, number of columns is capped at 2.
     */
    let linkCols = $derived.by(() => {
        const maxColumnLengthLargeWindow = 4;
        const maxNumberOfColumnsSmallWindow = 2;
        let result = [];
        let numberOfColumns: number;
        // Calculate number of columns, depending on window size.
        if (!windowIsSmall.current || links.length <= maxColumnLengthLargeWindow) {
            numberOfColumns = Math.ceil(links.length / maxColumnLengthLargeWindow);
        } else {
            numberOfColumns = maxNumberOfColumnsSmallWindow;
        }
        // Split links evenly over columns.
        let start: number,
            end = 0;
        for (let i = 0; i < numberOfColumns; i += 1) {
            start = end;
            end += Math.ceil((links.length - end) / (numberOfColumns - i));
            result.push(links.slice(start, end));
        }

        return result;
    });
    /** The link of the copyright notice */
    let copyrightLink = $derived(
        new FooterLink({
            href: m.uuIcsHref(),
            text: '© Utrecht University (ICS)',
            target: '_blank',
            localized: false
        })
    );
</script>

<footer style="transform: translateY({offset.current}px);" class="w-100vw">
    {#if !windowIsSmall.current}
        <div
            class="grid grid-flow-col grid-rows-1"
            style:grid-template-columns={`var(--spacing-footer-${linkCols.length <= 2 ? 'xl' : 'md'})
                auto
                repeat(${linkCols.length}, var(--spacing-footer-lg))
                auto
                var(--spacing-footer-${linkCols.length <= 2 ? 'xl' : 'md'})`}
        >
            {@render footerImage(logo)}
            {@render fillerDiv()}
            {@render footerLinks()}
            {@render fillerDiv()}
            {@render copyright()}
        </div>
    {:else}
        <div class="grid grid-flow-row grid-rows-[auto_auto]">
            <div class="flex flex-auto flex-row pt-padding-small pb-padding-small">
                {@render fillerDiv()}
                {@render footerLinks()}
                {@render fillerDiv()}
            </div>
            <div class="flex flex-auto flex-row">
                {@render footerImage(logo)}
                {@render fillerDiv()}
                {@render copyright()}
            </div>
        </div>
    {/if}
</footer>

<!-- Image linking to the home-page. -->
{#snippet footerImage(src: string)}
    <a href={appLink('/')} class="flex items-center">
        <img class="aspect-square w-footer-md not-md:w-footer-th" {src} alt="logo" />
    </a>
{/snippet}

<!-- Footer links divided over columns. -->
{#snippet footerLinks()}
    {#each linkCols as linkCol (linkCol)}
        <div class="flex flex-auto flex-col items-center justify-center">
            <div class="flex flex-col">
                {#each linkCol as link (link)}
                    <div class="p-0.5">
                        {@render footerLink(link)}
                    </div>
                {/each}
            </div>
        </div>
    {/each}
{/snippet}

<!-- Render a single footer link. -->
{#snippet footerLink(link: FooterLink)}
    <AnimatedUnderline --color-underline="var(--color-black)">
        {#if link.localized}
            <a href={appLink(link.href)} target={link.target}>
                {link.text}
            </a>
        {:else}
            <a rel="external" href={link.href} target={link.target}>
                {link.text}
            </a>
        {/if}
    </AnimatedUnderline>
{/snippet}

<!-- Render the copyright notice. -->
{#snippet copyright()}
    <div class="m-2 flex flex-col items-end justify-end">
        <div>
            {@render footerLink(copyrightLink)}
        </div>
    </div>
{/snippet}

<!-- Render an empty div that grows to take up space. -->
{#snippet fillerDiv()}
    <div class="flex flex-auto"></div>
{/snippet}

<style>
    @layer components {
        footer {
            margin-top: 2rem;
            background-color: var(--color-brown);
        }
    }
    /* Unused selector to load in theme variables. */
    :global(.load-footer-spacing) {
        padding: var(--spacing-footer-xl) var(--spacing-footer-lg) var(--spacing-footer-md)
            var(--spacing-footer-th);
    }
</style>
