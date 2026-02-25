<script lang="ts">
  import { selectedNode } from './selected-node.store';
  import Button from './Button.svelte';

  //create a new node for selected node
  function addCard() {
    if (!$selectedNode) return;
    $selectedNode.children.push({ card: 'New card', children: [] });
    selectedNode.set(null);
  }
</script>

<Button class="primary sm" onclick={addCard} disabled={!$selectedNode}>
  Add card
</Button>



