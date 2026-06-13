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
</script>
