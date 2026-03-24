<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { getJsonTest } from '$lib/backend/demo';
</script>

<div class="flex h-32 items-center justify-center">
    {#await getJsonTest()}
        <p>waiting on api...</p>
    {:then json_object}
        <p>Received: {JSON.stringify(json_object)}</p>
    {:catch error}
        <p style="color: red">{error.message}</p>
    {/await}
</div>
