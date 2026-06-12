<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render a button to change page size and a button to open filters in a sidebar.

    The two filter-related params are optional, but if one is included, both should be included
    to avoid unintended behaviour.

    The `pageSize` field should be bound to be updated properly.

    ## Params
    - `pageSize: string` - The bindable current page size.
    - `handlePageSizeChange: () => void` - The function to run when updating the page size.
    - `filterFields: Snippet | null` - Optional contents of the filter form.
                                       Placed inside of a form element.
    - `handleApplyFilters: (() => void) | null` - Optional function which is run upon confirmation of the filters.
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import { Drawer } from 'flowbite-svelte';
    import { FilterOutline } from 'flowbite-svelte-icons';
    import type { Snippet } from 'svelte';

    let {
        pageSize = $bindable(),
        handlePageSizeChange,
        filterFields = null,
        handleApplyFilters = null
    }: {
        pageSize: number;
        handlePageSizeChange: () => void;
        filterFields: Snippet | null;
        handleApplyFilters: (() => void) | null;
    } = $props();

    let filtersShown = $state(false);

    function handleFilterToggle() {
        filtersShown = !filtersShown;
    }

    function handleSubmit(e: SubmitEvent) {
        e.preventDefault();

        handleFilterToggle();
        handleApplyFilters!();
    }
</script>

<div class="flex w-full flex-col">
    <form class="mb-2 flex w-full flex-row items-center justify-between">
        {#if filterFields !== null}
            <button
                type="button"
                class={[
                    'ml-1 flex h-10 w-16 cursor-pointer items-center justify-center rounded-md border-2',
                    'border-gray-500 transition-colors duration-200 ease-in-out hover:bg-neutral-100'
                ]}
                onclick={handleFilterToggle}
            >
                <FilterOutline class="h-7 w-7 shrink-0" />
            </button>
        {/if}
        <div class="flex flex-row gap-2">
            <label for="page-size" class="flex items-center">
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

<Drawer bind:open={filtersShown}>
    <h3 class="page__subheading -mt-0.5 mb-4">{m.filters()}</h3>
    <form class="completed flex w-full flex-col gap-2" onsubmit={handleSubmit}>
        {@render filterFields!()}
        <button type="submit">{m.applyFilters()}</button>
    </form>
</Drawer>
