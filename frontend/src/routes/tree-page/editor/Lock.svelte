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
    import type { UserId } from '$lib/api-wrapper/users';
    import type { TreeId } from '$lib/backend/trees';
    import { onMount } from 'svelte';

    let { treeId }: { treeId: TreeId } = $props();

    type FrontendMessage = FrontendMessageForce | FrontendMessageForceSetActive;
    type FrontendMessageForce = {
        type: 'force';
    };
    type FrontendMessageForceSetActive = {
        type: 'setActive';
        newActiveState: boolean;
    };

    type BackendMessage =
        | BackendMessageLockAchieved
        | BackendMessageLockFailed
        | BackendMessageKickedOutBy;
    type BackendMessageLockAchieved = {
        type: 'lockAchieved';
    };
    type BackendMessageLockFailed = {
        type: 'lockFailed';
        activeUser: UserId;
    };
    type BackendMessageKickedOutBy = {
        type: 'kickedOutBy';
        userId: UserId;
    };

    // We can reuse some backend messages to prevent having to create more types for them.
    type LocalState =
        | LocalStateWaiting
        | LocalStateServerDisconnected
        | BackendMessageLockAchieved
        | BackendMessageLockFailed
        | BackendMessageKickedOutBy;
    type LocalStateWaiting = {
        type: 'waiting';
    };
    type LocalStateServerDisconnected = {
        type: 'serverDisconnected';
    };

    let localState: LocalState = $state({ type: 'waiting' });
    let hasLock = $derived(localState.type === 'lockAchieved');

    let protocol = $derived(window.location.protocol === 'http:' ? 'ws' : 'wss');
    let url = $derived(
        `${protocol}://${window.location.host}/api/v1/trees/${treeId}/aquire-edit-lock`
    );
    let websocket: WebSocket;

    onMount(() => {
        websocket = new WebSocket(url);
        websocket.onmessage = (event) => {
            let message: BackendMessage = JSON.parse(event.data);
            localState = message;
        };

        websocket.onclose = () => {
            if (localState.type !== 'kickedOutBy') {
                localState = { type: 'serverDisconnected' };
            }
        };

        return () => websocket.close();
    });

    function sendMessage(message: FrontendMessage) {
        websocket.send(JSON.stringify(message));
    }
</script>

<svelte:document
    onvisibilitychange={() => {
        const newActiveState = !document.hidden;
        console.log(newActiveState);
        sendMessage({ type: 'setActive', newActiveState });
    }}
/>

<!-- Temporary debugging -->
<div class="flex flex-col gap-2">
    <p class="font-bold" class:text-red-600={!hasLock} class:text-green-600={hasLock}>
        {JSON.stringify(localState)}
    </p>
    <button
        class="rounded bg-blue-300 p-1"
        class:hidden={localState.type !== 'lockFailed'}
        onclick={() => {
            sendMessage({ type: 'force' });
        }}
    >
        Force kick
    </button>
</div>
