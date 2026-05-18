<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--component
    This components tries to acquire an editing lock from the backend,
    to ensure no other user (or the same user on a different tab) is editing a tree at the same time.
 -->
<script lang="ts">
    import type { TreeId } from '$lib/backend/trees';
    import { onMount } from 'svelte';

    let { treeId }: { treeId: TreeId } = $props();

    let protocol = $derived(window.location.protocol === 'http:' ? 'ws' : 'wss');
    let url = $derived(
        `${protocol}://${window.location.host}/api/v1/trees/${treeId}/aquire-edit-lock`
    );
    let debugValue = $state('No websocket yet');

    onMount(() => {
        let websocket = new WebSocket(url);
        websocket.onopen = () => (debugValue = 'Websocket activated!');
        websocket.onmessage = (event) => (debugValue = `Websocket got message: ${event.data}`);
        websocket.onclose = () => (debugValue = 'Websocket closed!');

        return () => websocket.close();
    });
</script>

<!-- Temporary debugging -->
<p class="font-bold text-red-600">{debugValue}</p>
