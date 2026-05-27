<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--component
    This components tries to acquire an editing lock from the backend,
    to ensure no other user (or the same user on a different tab) is editing a tree at the same time.

    Once the lock is aquired, the `children` will be mounted to DOM and rendered in place of this component.
 -->
<script lang="ts">
    import type { UserId } from '$lib/api-wrapper/users';
    import type { TreeId } from '$lib/api-wrapper/trees';
    import { onMount, type Snippet } from 'svelte';
    import { m } from '$lib/paraglide/messages';
    import UserDisplay from '$lib/UserDisplay.svelte';

    let {
        treeId,
        /** These children will only be mounted if the lock is aquired */
        children
    }: { treeId: TreeId; children: Snippet } = $props();

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

    let localState = $state({ type: 'waiting' } as LocalState);

    let protocol = $derived(window.location.protocol === 'http:' ? 'ws' : 'wss');
    let url = $derived(
        `${protocol}://${window.location.host}/api/v1/trees/${treeId}/acquire-edit-lock`
    );
    let websocket: WebSocket;

    function initializeWebsocket() {
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
    }

    function retry() {
        localState = { type: 'waiting' };

        // Artificial delay so that the user actually sees that there is a retry
        setTimeout(initializeWebsocket, 200);
    }

    onMount(() => {
        initializeWebsocket();

        return () => websocket.close();
    });

    function sendMessage(message: FrontendMessage) {
        websocket.send(JSON.stringify(message));
    }
</script>

<svelte:document
    onvisibilitychange={() => {
        if (websocket.readyState === WebSocket.CONNECTING) {
            // When lock is first mounted, the websocket is not yet connected,
            // but this event will fire and say that the document is visible.
            return;
        }

        const newActiveState = !document.hidden;
        sendMessage({ type: 'setActive', newActiveState });
    }}
/>

{#if localState.type === 'waiting'}
    <p class="w-full text-center text-green-800">{m.loadingTree()}</p>
{/if}

{#if localState.type !== 'lockAchieved' && localState.type !== 'waiting'}
    <div class="m-3 flex flex-col items-center gap-2">
        <p>
            {#if localState.type === 'kickedOutBy'}
                Kicked by <UserDisplay userId={localState.userId} />
            {:else if localState.type === 'serverDisconnected'}
                Server disconnected!
            {:else if localState.type === 'lockFailed'}
                <UserDisplay userId={localState.activeUser} /> is currently editing this tree! You can
                force them out, but there could be unsaved changes.
            {/if}
        </p>
        <div class="flex gap-2">
            {#if localState.type === 'lockFailed'}
                <button
                    class="rounded bg-red-300 p-1"
                    onclick={() => {
                        sendMessage({ type: 'force' });
                    }}
                >
                    Force kick
                </button>
            {/if}
            <button class="rounded bg-blue-300 p-1" onclick={retry}> Retry </button>
        </div>
    </div>
{/if}

{#if localState.type === 'lockAchieved'}
    {@render children()}
{/if}
