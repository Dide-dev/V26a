<script lang="ts">
  import { selectedNode, secondSelectedNode } from './selected-node.store';

  function addCard() {
    if (!$selectedNode) return;

    if ($secondSelectedNode) {
        //try parent to child and child to parent combinations
        const firstIsParent = $selectedNode.children.includes($secondSelectedNode);
        const parent = firstIsParent ? $selectedNode : $secondSelectedNode;
        const child = firstIsParent ? $secondSelectedNode : $selectedNode;

        //inserts new node between parent and child combination
        const newNode = { card: 'New card', children: [child] };
        parent.children = parent.children.map(c => c === child ? newNode : c);
        secondSelectedNode.set(null);
    } else {
        //attach new node to selected node
        $selectedNode.children = [...$selectedNode.children, { card: 'New card', children: [] }];
    }

    selectedNode.set(null);
}

</script>

<button class="w-fit bg-blue-500 text-white px-3 py-1 rounded" onclick={addCard} disabled={!$selectedNode}>
  Add card
</button>



