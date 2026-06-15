<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import SubtreeEditor from './SubtreeEditor.svelte';
    import TreeEditButtons from './TreeEditButtons.svelte';
    import AddAndDrop from './AddAndDrop.svelte';
    import CardViewer from './CardViewer.svelte';
    import { followIndex, type TreeHistory } from './history.svelte';
    import type { SideBar } from './Sidebar.svelte';
    import RootNodeEditor from './RootNodeEditor.svelte';
    import SubtreeViewer from '../SubtreeViewer.svelte';
    import LineDrawer from '../LineDrawer.svelte';
    import type { Node, RootNode } from '$lib/api-wrapper/trees';
    import type { BranchColor } from '$lib/treePage';

    let {
        history = $bindable(),
        location,
        /**
         * Is set to true if this node is not present in `history.currentLocation`.
         * In that case this component is just being used for mock displaying the new card to be created.
         */
        fakeNode = false,
        dragState = $bindable(),
        sidebar = $bindable(),
        color
    }: {
        history: TreeHistory;
        location: number[];
        fakeNode?: boolean;
        dragState: DragState;
        sidebar: SideBar;
        color: BranchColor;
    } = $props();

    /** This will be null if `fakeNode` is set to true. */
    let node: null | RootNode | Node = $derived.by(() => {
        if (fakeNode) return null;

        return followIndex(history.currentStructure, location);
    });
    let isFirstChild = $derived(
        // Extra care here to detect if we are not the first child because of a fake node above here being inserted,
        // but the fake node itself should still have it set to true.
        location[location.length - 1] === 0 &&
            (fakeNode ||
                sidebar.type !== 'NewCard' ||
                JSON.stringify(location) !== JSON.stringify(sidebar.location))
    );
    let nextSiblingIndex = $derived([...location.slice(0, -1), location[location.length - 1] + 1]);

    /** All HTML children, inclusive possibly any fake nodes. */
    type Child = {
        index: number;
        isFake: boolean;
    };
    let children = $derived.by(() => {
        if (node === null) return [];
        let iterator = node.children.keys().map((index) => ({ index, isFake: false }));
        let list: Child[] = [...iterator];
        if (
            sidebar.type === 'NewCard' &&
            JSON.stringify(sidebar.location.slice(0, -1)) == JSON.stringify(location)
        ) {
            let lastIndex = sidebar.location[sidebar.location.length - 1];
            list.splice(lastIndex, 0, { index: lastIndex, isFake: true });
        }
        return list;
    });

    export type DragState = null | DragStateDragging | DragStateDismissed;
    export type DragStateDragging = {
        type: 'dragging';

        nodeToMove: number[];

        mouseX: number;
        mouseY: number;
    };
    /**
     * Used when the user is dragging, but the specific AddAndDrop instance is not a valid drop box.
     * This is to prevent a node from being dragged into itself and disappearing from the tree.
     */
    export type DragStateDismissed = {
        type: 'dismissed';
    };

    // You cannot drag a parent into its children.
    // So if this component is being dragged, don't tell its children.
    let iAmBeingDragged = $derived(
        dragState?.type === 'dragging' &&
            JSON.stringify(dragState.nodeToMove) === JSON.stringify(location)
    );
    // If next node is dragged there is no need to activate our own AddAndDrop under the card,
    // as it would not change anything for the user to drop it there.
    let nextIsBeingDragged = $derived(
        dragState?.type === 'dragging' &&
            JSON.stringify(dragState.nodeToMove) === JSON.stringify(nextSiblingIndex)
    );
    function dragStateChildrenSet(v: DragState) {
        /// The AddAndDrop will check if it's DragState is correct before calling this 'set' function.
        dragState = v;
    }
    function dragStateChildrenGetSelf(): DragState {
        return iAmBeingDragged || nextIsBeingDragged ? { type: 'dismissed' } : dragState;
    }
    function dragStateChildrenGetChildren(): DragState {
        return iAmBeingDragged ? { type: 'dismissed' } : dragState;
    }

    /**
     * Is used only when this is not a RootNode,
     * for giving to the AddAndDrop so it knows where to draw.
     */
    let heightOfOwnCard = $state(0);
</script>

<!--
    Padding on the top is to make room for a AddAndDrop button above here,
    only if this is a root node or I am not the first child.
-->
<div class="relative flex" class:mt-7={(node !== null && 'label' in node) || !isFirstChild}>
    <!--
        The following if statement and everything contained within,
        is for the dragging of a subtree.

        Render copy of self (but transparent and smaller) where the mouse currently is.
        We check again for the `dragState.type`, because otherwise typescript doesn't know
        that `dragState.mouseX` and `dragState.mouseY` exists.
    -->
    {#if iAmBeingDragged && dragState?.type === 'dragging'}
        <div
            class="transform:scale(0.7) pointer-events-none fixed z-20 opacity-70"
            style="left: {dragState.mouseX}px; top: {dragState.mouseY}px;"
        >
            <SubtreeViewer
                node={node!}
                cards={history.currentCards}
                {color}
                distanceFromRevealed={1}
                isRevealed={false}
            />
        </div>
    {/if}

    {#if isFirstChild && (node === null || 'cardId' in node)}
        <AddAndDrop
            {history}
            {location}
            bind:dragState={dragStateChildrenGetChildren, dragStateChildrenSet}
            lineLayout={{ type: 'over' }}
            bind:sidebar
        />
    {/if}

    {#if node !== null && 'label' in node}
        <!-- Root Node -->
        <RootNodeEditor {history} index={location[0]} />
    {:else}
        <!-- Standard Node -->
        {#if dragState === null}
            <TreeEditButtons {history} {location} bind:dragState bind:sidebar />
        {/if}
        <div class="flex flex-col" bind:clientHeight={heightOfOwnCard}>
            <div class="shrink-0">
                <CardViewer
                    card={sidebar.kind === 'CardEditor' && (fakeNode ||
                    (sidebar.type === 'EditCard' &&
                        JSON.stringify(sidebar.location) === JSON.stringify(location)))
                        ? sidebar.editingVersion
                        : history.currentCards[node!.cardId]}
                />
            </div>
            <AddAndDrop
                {history}
                location={nextSiblingIndex}
                bind:dragState={dragStateChildrenGetSelf, dragStateChildrenSet}
                lineLayout={{
                    type: 'under',
                    lineHeight: heightOfOwnCard
                }}
                bind:sidebar
            />
        </div>
    {/if}

    {#if children.length > 0}
        <LineDrawer alwaysFullWidth={true}>
            <!-- Here the html gets recursive as we render the 'node.children' -->
            {#each children as child (child)}
                {#if child.isFake}
                    <!-- This is the new dummy card currently being created -->
                    <SubtreeEditor
                        fakeNode={true}
                        bind:history
                        location={[...location, child.index]}
                        dragState={{ type: 'dismissed' }}
                        bind:sidebar
                        {color}
                    />
                {:else}
                    <SubtreeEditor
                        bind:history
                        location={[...location, child.index]}
                        bind:dragState={dragStateChildrenGetChildren, dragStateChildrenSet}
                        bind:sidebar
                        {color}
                    />
                {/if}
            {/each}
        </LineDrawer>
    {:else}
        <div class="flex h-9 items-center">
            <AddAndDrop
                {history}
                location={[...location, 0]}
                bind:dragState={dragStateChildrenGetChildren, dragStateChildrenSet}
                lineLayout={{ type: 'right' }}
                bind:sidebar
            />
        </div>
    {/if}
</div>
