<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { logout } from '$lib/api-wrapper/auth';
    import { m } from '$lib/paraglide/messages.js';

    onMount(async () => {
        await logout();
        goto('/');
    });
</script>

<h1>{m.loading()}</h1>
