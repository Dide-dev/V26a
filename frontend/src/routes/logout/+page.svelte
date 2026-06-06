<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { logout } from '$lib/api-wrapper/auth';

    onMount(async () => {
        await logout();
        goto('/');
    });
</script>
