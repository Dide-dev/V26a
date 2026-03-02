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
