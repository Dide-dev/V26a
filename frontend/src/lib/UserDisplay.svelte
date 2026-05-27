<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Displays the nickname of a user given the `user_id`.

-->

<script lang="ts">
    import { onMount } from 'svelte';
    import type { UserId } from './api-wrapper/users';

    let { userId }: { userId: UserId } = $props();

    let username = $state('Username loading...');
    onMount(() => {
        // TODO: actually get the nickname from a backend call,
        // when that gets created.
        // maybe also show real name on hover, make a link to profile page, show if user is admin, ...
        username = userId.toString();
    });
</script>

<span class="underline">{username}</span>
