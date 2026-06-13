<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import type { FilterOptions } from '$lib/api-wrapper/trees';
    import TreesList from '../trees/TreesList.svelte';

    let filterOptions: FilterOptions = $state({
        ownerName: null,
        searchTerms: null,
        book: [],
        tag: [],
        languageCode: null,
        subject: null,
        createdAfter: null,
        createdBefore: null,
        showOwnPrivate: true,
        editable: true,
        showPrivate: null
    });
</script>

<TreesList {filterOptions} titleText={m.myTrees()} />
