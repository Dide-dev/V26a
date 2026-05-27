<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Displays the name and/or nickname of a user given the `user_id`, as an inline html element.
-->

<script lang="ts">
    import { onMount } from 'svelte';
    import { getPublicUserData, type UserId } from './api-wrapper/users';
    import { m } from './paraglide/messages';

    let { userId }: { userId: UserId } = $props();

    type DisplayData = {
        displayText: string;
        hoverText: string | null;
    };

    let data: DisplayData = $state({
        displayText: `${m.loading()}...`,
        hoverText: `${m.loading()}...`
    });

    onMount(async () => {
        let userData = await getPublicUserData(userId);

        if (userData.nickname.trim() === '') {
            data = {
                displayText: userData.name,
                hoverText: null
            };
        } else {
            data = {
                displayText: userData.nickname,
                hoverText: userData.name
            };
        }
    });
</script>

<!-- Once a profile page for users is created, this could be turned into a link to it. -->
<span class="underline" title={data.hoverText}>{data.displayText}</span>
