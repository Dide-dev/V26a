<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { onMount } from 'svelte';
    import { redirectLogin } from '$lib/api-wrapper';
    import { getUserInfo, type UserInfo } from '$lib/api-wrapper/users';
    import { m } from '$lib/paraglide/messages.js';
    import { page } from '$app/state';

    let userInfo: UserInfo | null = $state(null);

    onMount(async () => {
        userInfo = getUserInfo();
        if (userInfo === null) {
            await redirectLogin(page.url);
            return;
        }
    });
</script>

<div>
    <h1 class="page__title">{m.classroomDashboard()}</h1>
</div>
