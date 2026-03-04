<script lang="ts">
    import type { Node } from '../tree-viewer/example_tree';

    //the selected nodes are iniatilized as properties
    //TODO, if a new card is send back then type & argument needs to be edited
     let { selectedNode, secondSelectedNode, onAdd }: {
        selectedNode: Node | null;
        secondSelectedNode: Node | null;
        onAdd: () => void;
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

        onAdd();
    }
    
</script>

<button
    class="w-fit rounded bg-blue-500 px-3 py-1 text-white"
    onclick={addCard}
    disabled={!selectedNode}>
        Add card
</button>
