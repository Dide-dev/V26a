<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render a semi-sticky navigation-bar with the provided links.

    ## Reactivity
    The nav-bar renders differently depending on screen size:
    - **Larger windows (>= 48rem):** Top-level links get aligned horizontally, with `leftLinks`
      aligned to the left, and `rightLinks` aligned to the right. Width gets determined by the
      `width` prop. Mega-drop-down menu opens when a top-level link with children is hovered.
    - **Smaller windows (< 48rem):** All links are hidden by default under a hamburger menu.
      When menu is opened, top-level links get aligned vertically. Top-level links with children
      are rendered as a drop-down, which will open up when clicked. The `defaultHref` of these links
      is therefore unreachable in smaller windows, and this field should **never** be unique among
      its children.

    ## Params
    - `leftLinks: NavLink[]`  - The links to align to the left on larger screens.
    - `rightLinks: NavLink[]` - The links to align to the right on larger screens.
    - `offsets: Tween<number>[]` - Bindable parameter which contains the current offset each
                                   top-level link on the nav-bar requires. Use it to offset the rest
                                   of the page accordingly, by taking its sum.
-->

<script lang="ts">
    import { appLink } from '$lib';
    import { MediaQuery } from 'svelte/reactivity';
    import { type NavChildLink, NavLink } from '$lib/navigation';
    import { cubicOut } from 'svelte/easing';
    import { Tween } from 'svelte/motion';
    import { onMount } from 'svelte';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';

    let {
        leftLinks,
        rightLinks,
        offsets = $bindable()
    }: {
        /** The links that are aligned to the left on larger screens. */
        leftLinks: NavLink[];
        /** The links that are aligned to the right on larger screens. */
        rightLinks: NavLink[];
        /**
         * The offsets required by each top-level link.
         * Its sum is used outside this component to match up rendering offset.
         */
        offsets: Tween<number>[];
    } = $props();

    /** The highest number of children of any top-level link. */
    let maxChildren: number = $derived(
        Math.max(...leftLinks.concat(rightLinks).map((link) => link.children.length))
    );
    /** Whether the hamburger-menu or dropdown is opened. */
    let menu = $state(false);
    /** Query whether the window is large or small. 'true' if large, and 'false' if not. */
    const windowIsLarge = new MediaQuery('min-width: 48rem');
    /**
     * The index of the top-level link, of which the drop-down is open. Only used in a smaller window.
     * Is `null` if no drop-down is open.
     */
    let openDropdownIndex: number | null = $state(null);
    /** The settings used for transitioning between animation states. */
    let animationFlow = {
        duration: 200,
        easing: cubicOut
    };
    /**
     * The animation states used for each top-level link with a drop-down. Only used in a smaller window.
     * When setting the `target` value on any index, that drop-down icon will change over a short duration.
     */
    let dropdownRotationStates = Array.from(
        // Lazy evaluation of `leftLinks` and `rightLinks`.
        { length: (() => leftLinks.length + rightLinks.length)() },
        () => new Tween(45, animationFlow)
    );
    /**
     * The animation states used for each child-link of each top-level link in a drop-down
     * to animate drop-down. Only used in a smaller window. When setting the `target` value
     * on any index, that child-link will move down or up over a short duration.
     */
    let dropdownDropStates = $state(
        // Lazy evaluation of `leftLinks` and `rightLinks`.
        (() =>
            leftLinks
                .concat(rightLinks)
                .map((link) =>
                    Array.from({ length: link.children.length }, () => new Tween(0, animationFlow))
                ))()
    );
    /** Most recently recorded scroll position on window. */
    let lastScrollPosition = $state(0);
    /** Margin between nav-bar and the top of the screen in pixels. When 0, nav-bar is fully visible. */
    let topScrollMargin = $state(0);
    /** Height of a single row in the nav-bar in pixels. */
    const navRowHeight = 40;
    /** Default scroll distance in browser in pixels.*/
    const defaultScrollDistance = 100;
    /** Maximum margin between nav-bar and the top of the screen in pixels. */
    const maxScrollMargin = navRowHeight + defaultScrollDistance;

    /** Toggle whether the menu is opened. */
    function toggleMenu() {
        if (menu) {
            closeMenu();
            return;
        }

        menu = true;
    }
    /** Close the menu or navigation drop-down. */
    function closeMenu() {
        menu = false;
        topScrollMargin = 0;
        openDropdownIndex = null;
        resetAnimationStates();
    }
    /** Reset the rotation for each drop-down to display a plus. Only used in a smaller window. */
    function resetAnimationStates() {
        dropdownRotationStates.forEach((tween) => (tween.target = 45));
        dropdownDropStates = dropdownDropStates.map(
            (arr) => (arr = arr.map((_, i) => new Tween(++i * -navRowHeight, animationFlow)))
        );
        offsets = offsets.map(() => new Tween(0, animationFlow));
    }
    /** Toggle whether a top-level's drop-down is open. Only used in a smaller window. */
    async function toggleDropdown(index: number) {
        // Play the closing animations on the drop-down that gets closed, if any.
        if (openDropdownIndex !== null) {
            // We don't await, because we want the rest of the function to continue running.
            closeDropdown(openDropdownIndex);
        }
        if (openDropdownIndex === index) {
            // The already open drop-down gets toggled, thus all drop-downs are closed.
            openDropdownIndex = null;
        } else {
            // Open a previously closed drop-down.
            openDropdownIndex = index;
            dropdownRotationStates[index].target -= 45;
            dropdownDropStates[index].forEach((tween) => (tween.target = 0));
            // Only restart animation fully, if previously finished.
            if (offsets[index].current === 0) {
                offsets[index] = new Tween(
                    dropdownDropStates[index].length * -navRowHeight,
                    animationFlow
                );
            }
            offsets[index].target = 0;
        }
    }
    /** Play the animations to close the dropdown of a top-level link. Only used in smaller windows. */
    async function closeDropdown(index: number) {
        // Return from cross to plus.
        dropdownRotationStates[index].target += 45;
        // Move up all child-links.
        dropdownDropStates[index].forEach((tween, i) => (tween.target = ++i * -navRowHeight));
        // Animate decreasing offset.
        await offsets[index].set(dropdownDropStates[index].length * -navRowHeight);
        // Re-align offset once children are no longer rendered (and thus don't take up space).
        offsets[index] = new Tween(0, animationFlow);
    }
    /** Calculate the offset up until the given index */
    function offsetAtIndex(index: number) {
        return offsets.slice(0, index).reduce((acc, curr) => acc + curr.current, 0);
    }

    // Upon loading the page, reset the animation states to their default states,
    // and load the current scroll position.
    onMount(() => {
        offsets = leftLinks.concat(rightLinks).map(() => new Tween(0, animationFlow));
        resetAnimationStates();
        lastScrollPosition = window.scrollY;
    });
</script>

<nav
    class={['sticky z-100 mb-6 text-neutral-100 not-md:justify-end']}
    style:top={menu ? '0px' : `${topScrollMargin}px`}
    onmouseleave={() => {
        if (!windowIsLarge.current) return;
        closeMenu();
    }}
>
    <!-- The top-level links, including the children in case of a smaller window. -->
    <div class={[menu && 'link-hovered', 'nav__bar flex not-md:flex-col']}>
        {#if !windowIsLarge.current}
            {@render hamburgerMenuButton()}
            <div class={['h-0 bg-white', menu && 'h-0.5']}></div>
        {/if}
        {#if menu || windowIsLarge.current}
            {#each leftLinks as link, i (link)}
                {@render navbarItem(link, i)}
            {/each}
            {#if windowIsLarge.current}
                <div class="min-w-0 flex-auto p-0"></div>
            {/if}
            {#each rightLinks as link, i (link)}
                {@render navbarItem(link, i + leftLinks.length)}
            {/each}
        {/if}
    </div>
    <!-- The mega-drop-down in case of a larger window. -->
    {#if windowIsLarge.current}
        <!-- Calculate the dropdown column width to match the top-level link widths. -->
        <div
            class="nav__dropdown"
            style:grid-template-columns={'' +
                // The width of each of the left links.
                // Comment below makes sure the theme variables are loaded.
                // Can be --spacing-nav-md, --spacing-nav-th or --spacing-nav-xt
                leftLinks.reduce((acc: string, curr: NavLink) => {
                    return acc + `var(--spacing-nav-${curr.props.width}) `;
                }, '') +
                // Spacing between the left and right links.
                'auto ' +
                // The width of each of the right links.
                rightLinks.reduce((acc: string, curr: NavLink) => {
                    return acc + `var(--spacing-nav-${curr.props.width}) `;
                }, '')}
            style:grid-template-rows={`repeat(${maxChildren}, auto)`}
        >
            {#each leftLinks as link (link)}
                {@render navChildrenLargeWindow(link)}
            {/each}
            {#each { length: maxChildren }}
                <div class="flex-auto"></div>
            {/each}
            {#each rightLinks as link (link)}
                {@render navChildrenLargeWindow(link)}
            {/each}
        </div>
    {/if}
</nav>

<svelte:window
    onscroll={() => {
        // Calculate the new margin between the nav-bar and top of the screen,
        // by clamping between 0 and `-maxScrollMargin`.
        topScrollMargin = Math.min(
            0,
            Math.max(-maxScrollMargin, topScrollMargin + lastScrollPosition - window.scrollY)
        );
        lastScrollPosition = window.scrollY;
    }}
/>

<!-- Toggle button to open the nav-menu. Only used in a smaller window. -->
{#snippet hamburgerMenuButton()}
    <button
        class={[
            'flex justify-end font-bold md:w-screen',
            'bg-tree-625 hover:bg-tree-500 hover:text-white',
            'p-[var(--spacing-padding-small)_var(--spacing-padding-normal)]'
        ]}
        onclick={toggleMenu}
        aria-label="Toggle menu"
    >
        {#if menu}✕{:else}☰{/if}
    </button>
{/snippet}

<!-- Top-level nav-bar item. -->
{#snippet navbarItem(link: NavLink, i: number)}
    {#if windowIsLarge.current}
        {@render navbarItemLargeWindow(link)}
    {:else}
        {@render navbarItemSmallWindow(link, i)}
    {/if}
{/snippet}

<!-- Top-level nav-bar item in a large window. -->
{#snippet navbarItemLargeWindow(link: NavLink)}
    <a
        style="min-width: var(--spacing-nav-{link.props.width});
               max-width: var(--spacing-nav-{link.props.width});"
        href={appLink(link.defaultHref)}
        onclick={closeMenu}
        onpointerenter={() => {
            menu = true;
        }}
    >
        <AnimatedUnderline
            --color-underline="var(--color-white)"
            --weight-underline="calc(var(--spacing) / 2)"
        >
            <p
                class="m-[var(--spacing-padding-small)_var(--spacing-padding-normal)] overflow-hidden font-bold"
            >
                {link.text}
            </p>
        </AnimatedUnderline>
    </a>
{/snippet}

<!--
    Top-level nav-bar item in a small window.
    If `link` has no children, will render as a link.
    If `link` does have children, will render a button and the children which are hidden by default.
-->
{#snippet navbarItemSmallWindow(link: NavLink, i: number)}
    <div class="flex-auto flex-col">
        <div
            style="transform:
                translateY({offsetAtIndex(i)}px);"
            class="relative z-10 flex flex-auto flex-row bg-tree-625 hover:bg-tree-450"
        >
            {#if link.children.length > 0}
                {@render navbarDropdownButton(link, i)}
            {:else}
                <a class="flex-auto" href={appLink(link.defaultHref)} onclick={closeMenu}>
                    <AnimatedUnderline
                        --color-underline="var(--color-white)"
                        --weight-underline="calc(var(--spacing) / 2)"
                    >
                        <p
                            class="m-[var(--spacing-padding-small)_var(--spacing-padding-normal)] font-bold"
                        >
                            {link.text}
                        </p>
                    </AnimatedUnderline>
                </a>
            {/if}
        </div>
        <!-- Only render children if any exist and the drop-down is open. -->
        {#if link.children.length > 0 && dropdownDropStates[i][0].current > -navRowHeight}
            {@render navChildrenSmallWindow(link, i)}
        {/if}
    </div>
{/snippet}

<!-- A button to open and close the drop-down with children. Only used in a small window. -->
{#snippet navbarDropdownButton(link: NavLink, i: number)}
    <button
        class={[
            'flex flex-auto cursor-pointer flex-row justify-between',
            'p-[var(--spacing-padding-small)_var(--spacing-padding-normal)]',
            'font-bold'
        ]}
        onclick={() => toggleDropdown(i)}
    >
        <span>
            {link.text}
        </span>
        <span style="transform: rotate({dropdownRotationStates[i].current}deg);"> ✕ </span>
    </button>
{/snippet}

<!-- Render all children of a top-level link in a large window. -->
{#snippet navChildrenLargeWindow(link: NavLink)}
    {#each link.children as child, i (child)}
        {@render navChildLargeWindow(child, i, link.children.length, link.props.localeSwitch)}
    {/each}
    {#each { length: maxChildren - link.children.length }}
        <div class="flex-auto"></div>
    {/each}
{/snippet}

<!-- Render all children of a top-level link in a small window. -->
{#snippet navChildrenSmallWindow(link: NavLink, parentIndex: number)}
    {#each link.children as child, i (child)}
        {@render navChildSmallWindow(child, i, parentIndex, link.props.localeSwitch)}
    {/each}
{/snippet}

<!-- Render a single child of a top-level link in a large window. -->
{#snippet navChildLargeWindow(link: NavChildLink, i: number, n: number, localeSwitch: boolean)}
    <!--
        A locale-switching link is already localized, doing so again will break the link,
        therefore `appLink` is not used. `data-sveltekit-reload` is required to trigger a reload
        on the page which loads the new locale.
    -->
    {#if localeSwitch}
        <a
            class={[i === n - 1 && 'md:rounded-bl-lg', 'nav-child']}
            href={link.href}
            onclick={closeMenu}
            data-sveltekit-reload
        >
            {@render underlineText(link.text)}
        </a>
    {:else}
        <a
            class={[i === n - 1 && 'md:rounded-bl-lg', 'nav-child']}
            href={appLink(link.href)}
            onclick={closeMenu}
        >
            {@render underlineText(link.text)}
        </a>
    {/if}
{/snippet}

<!-- Render a single child of a top-level link in a small window. -->
{#snippet navChildSmallWindow(
    link: NavChildLink,
    i: number,
    parentIndex: number,
    localeSwitch: boolean
)}
    <!--
        A locale-switching link is already localized, doing so again will break the link,
        therefore `appLink` is not used. `data-sveltekit-reload` is required to trigger a reload
        on the page which loads the new locale.
    -->
    {#if localeSwitch}
        <a
            style="transform: translateY({dropdownDropStates[parentIndex][i].current +
                offsetAtIndex(parentIndex)}px); z-index: {9 - i};"
            class={['nav-child']}
            href={link.href}
            onclick={closeMenu}
            data-sveltekit-reload
        >
            {@render underlineText(link.text)}
        </a>
    {:else}
        <a
            style="transform: translateY({dropdownDropStates[parentIndex][i].current +
                offsetAtIndex(parentIndex)}px); z-index: {9 - i};"
            class={['nav-child']}
            href={appLink(link.href)}
            onclick={closeMenu}
        >
            {@render underlineText(link.text)}
        </a>
    {/if}
{/snippet}

<!-- Render text with an animated underline effect. -->
{#snippet underlineText(text: string)}
    <AnimatedUnderline
        --color-underline="var(--color-black)"
        --weight-underline="calc(var(--spacing) * 0.4)"
    >
        <p>
            {text}
        </p>
    </AnimatedUnderline>
{/snippet}

<style>
    @layer components {
        nav {
            @media (width >= 48rem) {
                /* Layout */
                display: grid;
                grid-template-rows: auto auto;
                position: relative;

                .nav__bar > * {
                    /* Layout */
                    min-width: var(--spacing-nav-normal);
                    text-align: center;

                    /* Formatting */
                    background-color: var(--color-tree-500);
                    background-image: linear-gradient(
                        to right in oklab,
                        var(--color-tree-600),
                        var(--color-tree-600),
                        var(--color-tree-625)
                    );

                    /* Fallback in case the browser does not support `oklab` gradients */
                    @supports not (background-image: linear-gradient(in lab, red, red)) {
                        background-image: linear-gradient(
                            to right,
                            var(--color-tree-600),
                            var(--color-tree-600),
                            var(--color-tree-625)
                        );
                    }

                    &:is(a):hover {
                        background-image: none;

                        color: var(--color-white);

                        + * {
                            background-image: linear-gradient(
                                to right in oklab,
                                var(--color-tree-625),
                                var(--color-tree-600),
                                var(--color-tree-625)
                            );

                            /* Fallback in case the browser does not support `oklab` gradients */
                            @supports not (background-image: linear-gradient(in lab, red, red)) {
                                background-image: linear-gradient(
                                    to right,
                                    var(--color-tree-625),
                                    var(--color-tree-600),
                                    var(--color-tree-625)
                                );
                            }
                        }
                    }
                }
            }
        }

        .nav__dropdown {
            /* Position */
            position: absolute;
            z-index: 100;
            top: 100%;
            left: 0;
            right: 0;

            /* Hidden by default */
            display: none;

            /* Grid layout */
            grid-auto-flow: column;

            /* Formatting */
            background-color: var(--color-tree-50);
            box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
        }

        /* Show dropdown when hovered */
        .nav__bar.link-hovered + .nav__dropdown {
            display: grid;
        }

        .nav-child {
            /* Layout */
            display: flex;
            position: relative;

            p {
                margin: var(--spacing-padding-small);
                @media (width < 48rem) {
                    margin: var(--spacing-padding-small) var(--spacing-padding-lg);
                }
            }

            /* Formatting */
            background-color: var(--color-tree-125);
            color: var(--color-neutral-600);
            font-weight: var(--font-weight-semibold);

            @media (width < 48rem) {
                &:nth-child(odd) {
                    background-color: var(--color-tree-150);
                }
            }

            &:hover {
                color: var(--color-black);
                background-color: var(--color-tree-200);
            }
        }
    }
</style>
