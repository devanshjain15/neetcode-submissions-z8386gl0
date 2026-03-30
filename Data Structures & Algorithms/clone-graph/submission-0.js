/**
 * // Definition for a Node.
 * class Node {
 *     constructor(val = 0, neighbors = []) {
 *       this.val = val;
 *       this.neighbors = neighbors;
 *     }
 * }
 */

class Solution {
    /**
     * @param {Node} node
     * @return {Node}
     */
    cloneGraph(node) {
        let clone = null;
        if (node) {
            function cloneAndConnect(
                node,
                clonedNodes = new Map()
            ) {
                if (!clonedNodes.has(node)) {
                    const clonedNode = new Node(node.val);
                    clonedNodes.set(node, clonedNode);
                    const neighbors = node.neighbors;
                    for (let i = 0; i < neighbors.length; i++) {
                        const neighbor = neighbors[i];
                        clonedNodes = cloneAndConnect(neighbor, clonedNodes);
                        const clonedNeighbor = clonedNodes.get(neighbor);
                        clonedNode.neighbors.push(clonedNeighbor);
                    }
                }
                return clonedNodes;
            }

            let clonedNodesMap = cloneAndConnect(node);
            clone = clonedNodesMap.get(node);
        }
        return clone;
    }
}
