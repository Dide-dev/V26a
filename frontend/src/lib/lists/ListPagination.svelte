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
    import { ChevronLeftOutline, ChevronRightOutline } from 'flowbite-svelte-icons';
    import { m } from '$lib/paraglide/messages.js';
    import type { Snippet } from 'svelte';
    import FilterContainer from '$lib/lists/FilterContainer.svelte';

    let {
        currentPage = $bindable(),
        totalPages,
        handlePageChange,
        pageSize = $bindable(),
        handlePageSizeChange,
        filterFields = null,
        handleApplyFilters = null,
        children
    }: {
        currentPage: number;
        totalPages: number;
        handlePageChange: () => void;
        pageSize: number;
        handlePageSizeChange: () => void;
        filterFields: Snippet | null;
        handleApplyFilters: (() => void) | null;
        children: Snippet;
    } = $props();
</script>

<FilterContainer bind:pageSize {handlePageSizeChange} {handleApplyFilters} {filterFields} />

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
