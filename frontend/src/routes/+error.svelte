<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { page } from '$app/state';
    import { m } from '$lib/paraglide/messages.js';
    import seal from '$lib/assets/sealOfConfusion.png';
</script>

{#if page.status === 404}
    <h1 class="page__title">{m.pageNotFound()}</h1>
{:else}
    <h1 class="page__title">Error: {page.status}</h1>
    <p>{page.error?.message}</p>
{/if}

<img class="aspect-square w-120 not-md:w-75" src={seal} alt="An error occurred." />
