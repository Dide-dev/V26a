import type { TreeWithCards, Node, RootNode } from './trees';
import type { CardGetData, CardId } from './cards';
import type {
    HeuristicTree,
    Node as HeuristicNode,
    RootNode as HeuristicRootNode
} from '../../routes/tree-viewer/example_tree';

/** Adapter-function to convert the tree structure from trees.ts to the HeuristicTree structure from example tree, used for NodeViewer. */
export function convertTree(tree: TreeWithCards): HeuristicTree {
    return {
        name: tree.data.title,
        assignment: tree.data.assignmentText ?? '',
        roots: tree.structure.rootNodes.map((root) => convertRootNode(root, tree.cards))
    };
}
function convertRootNode(root: RootNode, cards: Map<CardId, CardGetData>): HeuristicRootNode {
    return {
        label: root.label,
        children: root.children.map((child) => convertNode(child, cards))
    };
}
function convertNode(node: Node, cards: Map<CardId, CardGetData>): HeuristicNode {
    const card = cards.get(node.cardId);

    if (!card) {
        throw new Error(`Card ${node.cardId} not found in tree`);
    }

    return {
        card: card.front,
        children: node.children.map((child) => convertNode(child, cards))
    };
}
