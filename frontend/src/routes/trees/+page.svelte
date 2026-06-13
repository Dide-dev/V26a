<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import type { FilterOptions } from '$lib/api-wrapper/trees';
    import { getLocale } from '$lib/paraglide/runtime';
    import TreesList from './TreesList.svelte';

    let filterOptions: FilterOptions = $state({
        ownerName: null,
        searchTerms: null,
        book: [],
        tag: [],
        languageCode: getLocale(),
        subject: null,
        createdAfter: null,
        createdBefore: null,
        showOwnPrivate: true,
        editable: null,
        showPrivate: null
    });
</script>

<TreesList {filterOptions} titleText={m.allTrees()} />
