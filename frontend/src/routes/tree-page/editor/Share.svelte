<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import type { ManageSideBar } from './Sidebar.svelte';

    let {
        sidebar = $bindable()
    }: {
        sidebar: ManageSideBar | null;
    } = $props();
</script>
