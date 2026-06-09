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
    import Popup from '$lib/Popup.svelte';

    let {
        treeId,
        /** These children will only be mounted if the lock is aquired */
        children
    }: { treeId: TreeId; children: Snippet } = $props();

    type FrontendMessage = FrontendMessageForce;
    type FrontendMessageForce = {
        type: 'force';
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
        | LocalStateLockFailed
        | BackendMessageLockAchieved
        | BackendMessageKickedOutBy;
    type LocalStateWaiting = {
        type: 'waiting';
    };
    type LocalStateServerDisconnected = {
        type: 'serverDisconnected';
    };
    type LocalStateLockFailed = BackendMessageLockFailed & {
        popupOpen?: boolean;
    };

    let localState = $state({ type: 'waiting' } as LocalState);

    let websocket: WebSocket;

    function initializeWebsocket() {
        websocket = new WebSocket(`/api/v1/trees/${treeId}/edit-lock`);
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

    function sendForceMessage() {
        let message: FrontendMessage = { type: 'force' };
        websocket.send(JSON.stringify(message));
    }
</script>

{#if localState.type === 'waiting'}
    <p class="w-full text-center text-green-800">{m.loadingTree()}</p>
{/if}

{#snippet forceKickSnippet(state: LocalStateLockFailed)}
    {#if state.popupOpen ?? false}
        <Popup>
            <div class="flex max-w-md flex-col gap-2">
                <p>
                    <UserDisplay userId={state.activeUser} />
                    {m.otherUserKickConfirm()}
                </p>
                <div class="flex justify-end gap-2">
                    <button
                        class="cursor-pointer rounded bg-red-300 p-1"
                        onclick={sendForceMessage}
                    >
                        {m.forceKick()}
                    </button>
                    <button
                        class="cursor-pointer rounded bg-green-300 p-1"
                        onclick={() => {
                            // This check is unnecessary, but typescript doesn't realize that
                            if (localState.type !== 'lockFailed') return;

                            localState.popupOpen = false;
                        }}
                    >
                        {m.cancel()}
                    </button>
                </div>
            </div>
        </Popup>
    {/if}
{/snippet}

{#if localState.type !== 'lockAchieved' && localState.type !== 'waiting'}
    <div class="m-3 flex max-w-md flex-col items-center gap-2">
        <p>
            {#if localState.type === 'kickedOutBy'}
                {m.kickedBy()} <UserDisplay userId={localState.userId} />
            {:else if localState.type === 'serverDisconnected'}
                {m.serverDisconnected()}
            {:else if localState.type === 'lockFailed'}
                <UserDisplay userId={localState.activeUser} /> {m.otherUserHasLock()}
            {/if}
        </p>
        <div class="flex gap-2">
            <button class="cursor-pointer rounded bg-blue-300 p-1" onclick={retry}>
                {m.retry()}
            </button>
            {#if localState.type === 'lockFailed'}
                <button
                    class="cursor-pointer rounded bg-red-300 p-1"
                    onclick={() => {
                        // This check is unnecessary, but typescript doesn't realize that
                        if (localState.type !== 'lockFailed') return;

                        localState.popupOpen = true;
                    }}
                >
                    {m.forceKick()}
                </button>
                {@render forceKickSnippet(localState)}
            {/if}
        </div>
    </div>
{/if}

{#if localState.type === 'lockAchieved'}
    {@render children()}
{/if}
