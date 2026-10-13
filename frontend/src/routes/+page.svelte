<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { getLocale } from '$lib/paraglide/runtime';
    import { m } from '$lib/paraglide/messages.js';
    import { AccordionItem, Accordion } from 'flowbite-svelte';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';
    import { homePageCards } from '$lib/homeContents';

    // The contents of a card on the home-page.
    type CardContent = {
        title: string;
        color: string;
        plainContents: PlainContent[];
        popOpenContents: PopOpenContent[];
    };

    type PlainContent = string;

    // The contents of a pop-open item.
    type PopOpenContent = {
        heading: string;
        content: PlainContent[];
    };
</script>

<div class="flex w-full justify-center">
    <div class="flex w-full flex-col items-center justify-center">
        <h1 class="page__title mb-8">{m.heuristicTrees()}</h1>
        <div class="alternating__container flex-col">
            {#each homePageCards[getLocale()] as card (card)}
                {@render contentCard(card)}
            {/each}
        </div>
    </div>
</div>

{#snippet contentCard(content: CardContent)}
    <div class="flex" style={`--color-card: ${content.color}`}>
        <div class="card__container p-3">
            <div class="md:max-w-4/5">
                <h1 class="page__heading">{content.title}</h1>
                {#each content.plainContents as item (item)}
                    <p class="mb-1 last-of-type:mb-4">
                        <!-- We render rich text. -->
                        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                        {@html item}
                    </p>
                {/each}
            </div>
            <Accordion
                activeClass="bg-neutral-300 not-last:border-b-2!"
                inactiveClass="bg-white hover:bg-neutral-300 transition-colors duration-150 ease-in-out"
                class="overflow-hidden border-4 border-neutral-300 *:*:border-0! *:*:border-neutral-300! *:*:p-0!"
            >
                {#each content.popOpenContents as popOpenItem (popOpenItem)}
                    <AccordionItem
                        class="*:cursor-pointer *:rounded-none! *:border-0! *:p-0! *:pr-5! not-first:*:border-t-2! not-last:*:border-neutral-300! first:*:border-t-0!"
                    >
                        {#snippet header()}
                            <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                                <p class="m-5 text-left text-lg">{popOpenItem.heading}</p>
                            </AnimatedUnderline>
                        {/snippet}
                        <div class="bg-white p-2">
                            {#each popOpenItem.content as paragraph (paragraph)}
                                <p class="mb-1 text-left">
                                    <!-- We render rich text. -->
                                    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                                    {@html paragraph}
                                </p>
                            {/each}
                        </div>
                    </AccordionItem>
                {/each}
            </Accordion>
        </div>
    </div>
{/snippet}

<style>
    @layer components {
        .alternating__container {
            /* Layout */
            width: calc(2 / 3 * 100%);

            > * > * {
                width: calc(3 / 4 * 100%);
            }

            @media (width < 48rem) {
                width: calc(7 / 8 * 100%);

                > * > * {
                    width: calc(7 / 8 * 100%);
                }
            }

            > * {
                /* Layout */
                width: 100%;
                position: relative;
                margin-bottom: calc(var(--spacing) * 6);

                /* Card backdrop with offset */
                &::before {
                    content: '';
                    position: absolute;
                    z-index: -1;
                    height: calc(3 / 4 * 100%);
                    width: calc(3.3 / 5 * 100%);
                    @media (width < 48rem) {
                        width: calc(100% + var(--spacing) * 6);
                    }

                    /* Color provided in the style of the element */
                    background-color: var(--color-card);
                }

                /* Left-aligned cards */
                &:nth-child(odd) {
                    /* Layout */
                    justify-content: flex-start;

                    .page__heading {
                        text-align: left;

                        &::before {
                            left: calc(var(--spacing) * -3);
                        }
                    }

                    /* Offset to the left */
                    &::before {
                        top: calc(var(--spacing) * -2);
                        left: calc(var(--spacing) * -6);
                        border-radius: var(--radius-2xl) var(--radius-sm) var(--radius-2xl)
                            var(--radius-sm);

                        @media (width < 48rem) {
                            left: calc(var(--spacing) * -4);
                        }
                    }
                }

                /* Right-aligned cards */
                &:nth-child(even) {
                    /* Layout */
                    justify-content: flex-end;

                    .page__heading {
                        text-align: right;

                        &::before {
                            right: calc(var(--spacing) * -3);
                        }
                    }

                    /* Align content to the right */
                    .card__container {
                        display: flex;
                        flex-direction: column;
                        justify-content: right;
                        align-items: flex-end;
                    }

                    /* Offset to the right */
                    &::before {
                        top: calc(var(--spacing) * -2);
                        right: calc(var(--spacing) * -6);
                        border-radius: var(--radius-sm) var(--radius-2xl) var(--radius-sm)
                            var(--radius-2xl);

                        @media (width < 48rem) {
                            right: calc(var(--spacing) * -4);
                        }
                    }
                }
            }

            /* Card headings */
            .page__heading {
                position: relative;
                margin-bottom: var(--spacing-padding-small);

                /* Heading marker */
                &::before {
                    /* Layout */
                    content: '';
                    position: absolute;
                    bottom: 5%;
                    height: 87.5%;
                    z-index: 1;

                    /* Formatting */
                    background-color: var(--color-neutral-350);
                    width: var(--spacing-padding-minor);
                    transition: background-color 200ms ease-in-out;
                }

                &:hover::before {
                    background-color: var(--color-neutral-500);
                }
            }
        }
    }
</style>
