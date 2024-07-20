

class InterfaceHelpers 
{
    getConnectedNodes(nodes, edges, nodesData, startNodeId) 
    {
        const visitedNodes = new Set();
        const stack = [startNodeId];
        const connectedNodes = [];

        while (stack.length > 0) 
        {
            const nodeId = stack.pop();
            if (!visitedNodes.has(nodeId)) 
            {
                visitedNodes.add(nodeId);
                const currentNode = nodes.find(node => node.id === nodeId);
                if (currentNode) 
                {
                    const nodeData = nodesData.find(data => data.node === nodeId);
                    if (nodeData) { connectedNodes.push({ ...currentNode, data: nodeData.data }); }
                    else { connectedNodes.push(currentNode); }
                    const connectedEdges = edges.filter(edge => edge.source === nodeId || edge.target === nodeId);
                    connectedEdges.forEach(edge => 
                    {
                        if (edge.source === nodeId && !visitedNodes.has(edge.target)) { stack.push(edge.target); } 
                        else if (edge.target === nodeId && !visitedNodes.has(edge.source)) { stack.push(edge.source); }
                    });
                }
            }
        }
        return connectedNodes;
    }

    generateStructure(connectedNodes, edges) 
    {
        const modelNode = connectedNodes.find(node => node.type === 'modele_node');
        const functions = connectedNodes.filter(node => node.type === 'fonction_node');
        const parameters = connectedNodes.filter(node => 
            node.type === 'parameter_node' || 
            node.type === 'parameter_slider_node' || 
            node.type === 'parameter_slider_bool_node' || 
            node.type === 'parameter_slider_neuronnes_node' || 
            node.type === 'parameter_text_node' || 
            node.type === 'parameter_bool_node'
        );
        const datasetNode = connectedNodes.find(node => node.type === 'dataset_node');

        const getNodeData = (nodeId) => 
        {
            const node = connectedNodes.find(node => node.id === nodeId);
            return node ? node.data : null;
        };

        const getConnectedNodesData = (nodeId) => 
        {
            const connectedEdges = edges.filter(edge => edge.source === nodeId || edge.target === nodeId);
            const connectedNodeIds = connectedEdges.map(edge => edge.source === nodeId ? edge.target : edge.source);
            return connectedNodeIds.map(id => getNodeData(id)).filter(data => data);
        };

        const inputSize = datasetNode ? datasetNode.data.value.input_size : null;
        const outputSize = datasetNode ? datasetNode.data.value.output_size : null;

        const structure = {
            modele: modelNode.data.label,
            fonctions: functions.map(func => (
            {
                label: func.data.label,
                parametres: parameters.filter(param => 
                    edges.some(edge => 
                        (edge.source === func.id && edge.target === param.id) || 
                        (edge.target === func.id && edge.source === param.id)
                    )
                ).map(param => 
                {
                    if (param.data.label === 'Neuronnes') {
                        const connectedData = getConnectedNodesData(param.id)
                            .filter(data => data.label !== 'Initialisation')
                            .sort((a, b) => a.order - b.order);

                        const coucheSortie = connectedData.find(data => data.label === 'Couche de sortie');
                        let neuronnesData = connectedData.filter(data => data.label !== 'Couche de sortie')
                            .map(data => data.value || 0);

                        if (coucheSortie) { neuronnesData.push(coucheSortie.value || 0); }
                        if (inputSize !== null) { neuronnesData = [inputSize, ...neuronnesData]; }
                        if (outputSize !== null) { neuronnesData.push(outputSize); }

                        return { label: 'Neuronnes', value: neuronnesData };
                    }
                    
                    return {
                        label: param.data.label,
                        value: param.data.value || 0,
                        enable: param.data.enable || null 
                    };
                })
            }))
        };

        return structure;
    }
}

export default InterfaceHelpers;
