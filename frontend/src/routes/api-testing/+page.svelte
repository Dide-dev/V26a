<script lang="ts">
    import { get_json_test } from '$lib/backend/demo';
</script>

<div class="flex h-32 items-center justify-center">
    {#await get_json_test()}
        <p>waiting on api...</p>
    {:then json_object}
        <p>Received: {JSON.stringify(json_object)}</p>
    {:catch error}
        <p style="color: red">{error.message}</p>
    {/await}
</div>
