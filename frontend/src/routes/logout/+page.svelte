<script lang="ts">
    import { redirect } from '@sveltejs/kit';
    import { logout } from '$lib/api-wrapper/auth';

    export async function load() {
        await logout();
        redirect(302, '/');
    }
</script>