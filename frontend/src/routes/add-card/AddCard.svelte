<script lang="ts">
    import type { Node } from '../tree-viewer/example_tree';

    //attach a "bind" to the selected nodes
    let { selectedNode = $bindable(), secondSelectedNode = $bindable() }: 
    {
        selectedNode: Node | null;
        secondSelectedNode: Node | null;
    } = $props();

    function addCard() {
        if (!selectedNode) return;

        if (secondSelectedNode) {
            //try parent to child and child to parent combinations
            const firstIsParent = selectedNode.children.includes(secondSelectedNode);
            const parent = firstIsParent ? selectedNode : secondSelectedNode;
            const child = firstIsParent ? secondSelectedNode : selectedNode;

            //inserts new node between parent and child combination
            const newNode = { card: 'New card', children: [child as Node] };
            parent.children = parent.children.map(c => c === child ? newNode : c) as Node[];
            secondSelectedNode= null;
        } else {
            //attach new node to selected node
            selectedNode.children = [
                ...selectedNode.children,
                { card: 'New card', children: [] }
            ];
        }

        selectedNode = null;
    }
    
</script>

<button
    class="w-fit rounded bg-blue-500 px-3 py-1 text-white"
    onclick={addCard}
    disabled={!selectedNode}>
        {secondSelectedNode ? 'Add in between' : 'Add card'}
</button>
