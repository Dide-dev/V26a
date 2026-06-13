<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render the buttons to change pages and page size on a list page with pagination options.
    Optionally render a filters button if `filterFields` and `handleApplyFilters` are passed.

    The two filter-related params are optional, but if one is included, both should be included
    to avoid unintended behaviour.

    The `currentPage` and `pageSize` fields should be bound to be updated properly.

    ## Params
    - `title: Snippet` - The page title.
    - `currentPage: number` - The bindable current page index.
    - `totalPages: number` - The total number of pages.
    - `handlePageChange: () => void` - The function to run when updating the `currentPage`.
    - `pageSize: string` - The bindable current page size.
    - `handlePageSizeChange: () => void` - The function to run when updating the page size.
    - `filterFields: Snippet | null` - Optional contents of the filter form.
                                       Placed inside of a form element.
    - `handleApplyFilters: (() => void) | null` - Optional function which is run upon confirmation of the filters.
    - `children: Snippet` - The contents of the pagination to be rendered based on the page and page size.
-->

<script lang="ts">
    import { PaginationNav } from 'flowbite-svelte';
    import { ChevronLeftOutline, ChevronRightOutline, FilterOutline } from 'flowbite-svelte-icons';
    import { m } from '$lib/paraglide/messages.js';
    import type { Snippet } from 'svelte';
    import { Tween } from 'svelte/motion';
    import { cubicOut } from 'svelte/easing';

    let {
        title,
        currentPage = $bindable(),
        totalPages,
        handlePageChange,
        pageSize = $bindable(),
        handlePageSizeChange,
        filterFields = null,
        handleApplyFilters = null,
        children
    }: {
        title: Snippet;
        currentPage: number;
        totalPages: number;
        handlePageChange: () => void;
        pageSize: number;
        handlePageSizeChange: () => void;
        filterFields?: Snippet | null;
        handleApplyFilters?: (() => void) | null;
        children: Snippet;
    } = $props();

    let filtersShown = $state(false);

    let filterFraction = new Tween(0, {
        duration: 400,
        easing: cubicOut
    });

    function handleFilterToggle() {
        filtersShown = !filtersShown;

        filterFraction.target = filtersShown ? 1 : 0;
    }

    function handleSubmit(e: SubmitEvent) {
        e.preventDefault();

        handleFilterToggle();
        handleApplyFilters!();
    }
</script>

<div class={['flex max-h-full w-7/8 max-w-3xl flex-col']}>
    <div class="list-grid" style={`--filter-col-width: ${filterFraction.current}fr`}>
        <div></div>
        <div>
            {@render title()}
            <div class="flex w-full flex-col items-center justify-center overflow-auto">
                <div class="flex w-full flex-col">
                    <form class="mb-2 flex w-full flex-row items-center justify-between gap-3">
                        {#if filterFields !== null}
                            <button
                                type="button"
                                class={[
                                    'ml-1 flex h-10 w-16 cursor-pointer items-center justify-center rounded-md border-2',
                                    'border-gray-500 bg-white transition-colors duration-200 ease-in-out hover:bg-neutral-100'
                                ]}
                                onclick={handleFilterToggle}
                                title={m.showFilters()}
                            >
                                <FilterOutline class="h-7 w-7 shrink-0" />
                            </button>
                        {/if}
                        <div class="flex flex-row gap-2">
                            <label for="page-size" class="flex items-center text-right">
                                {m.numberPerPage()}
                            </label>
                            <select
                                id="page-size"
                                class={[
                                    'input__select',
                                    'mr-1 flex h-10 w-20 cursor-pointer items-center justify-center rounded-md border-2',
                                    'border-gray-500 transition-colors duration-200 ease-in-out hover:bg-neutral-100'
                                ]}
                                bind:value={pageSize}
                                onchange={handlePageSizeChange}
                            >
                                <option value={5}> 5 </option>
                                <option value={10}>10</option>
                                <option value={20}>20</option>
                                <option value={50}>50</option>
                            </select>
                        </div>
                    </form>
                </div>
            </div>
        </div>

        <div class="overflow-x-hidden overflow-y-auto">
            {#if filterFields !== null}
                <div
                    class={[
                        !filtersShown && 'not-md:hidden',
                        !filtersShown && filterFraction.current === 0 && 'hidden'
                    ]}
                >
                    <h3 class="page__subheading">{m.filters()}</h3>
                    <form
                        class="completed filters flex w-full flex-col gap-2"
                        onsubmit={handleSubmit}
                    >
                        {@render filterFields!()}
                    </form>
                </div>
            {/if}
        </div>

        <div class="flex flex-col items-center justify-start overflow-y-auto">
            {@render children()}

            {#if totalPages > 0}
                <PaginationNav
                    {currentPage}
                    {totalPages}
                    onPageChange={(newPage: number) => {
                        currentPage = newPage;
                        handlePageChange();
                    }}
                >
                    {#snippet prevContent()}
                        <span class="sr-only">{m.previousPage()}</span>
                        <ChevronLeftOutline class="h-5 w-5" />
                    {/snippet}
                    {#snippet nextContent()}
                        <span class="sr-only">{m.nextPage()}</span>
                        <ChevronRightOutline class="h-5 w-5" />
                    {/snippet}
                </PaginationNav>
            {/if}
        </div>
    </div>
</div>

<style>
    @layer components {
        .list-grid {
            display: grid;
            grid-auto-flow: row;
            overflow: auto;
            align-items: start;

            gap: var(--spacing-padding-small);

            @media (min-width: 48rem) {
                grid-template-columns: var(--filter-col-width) 1.2fr;
            }

            @media (width < 48rem) {
                grid-template-columns: auto;
            }
        }

        .filters {
            :global(label) {
                text-wrap: nowrap;
            }
        }
    }
</style>
