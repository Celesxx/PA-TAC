import React from 'react';
import ReactFlow, { Controls, Background } from 'reactflow';
import 'reactflow/dist/style.css';
import "../../assets/css/blocks/interface.asset.css"

class Interface extends React.Component 
{
    constructor(props) 
    {
        super(props);
        this.state = { };
    }

    getConnectedNodes = (startNodeId) => 
    {
        const { nodes, edges, nodesData } = this.props;
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
                    if (nodeData) 
                    {
                        connectedNodes.push({ ...currentNode, data: nodeData.data });
                    }
                    else 
                    {
                        connectedNodes.push(currentNode);
                    }
                    const connectedEdges = edges.filter(edge => edge.source === nodeId || edge.target === nodeId);
                    connectedEdges.forEach(edge => 
                    {
                        if (edge.source === nodeId && !visitedNodes.has(edge.target)) 
                        {
                            stack.push(edge.target);
                        }
                        else if (edge.target === nodeId && !visitedNodes.has(edge.source)) 
                        {
                            stack.push(edge.source);
                        }
                    });
                }
            }
        }
        return connectedNodes;
    }
    

    generateStructure = (connectedNodes) => 
    {
        const modelNode = connectedNodes.find(node => node.type === 'modele_node');
        const functions = connectedNodes.filter(node => node.type === 'fonction_node');
        const parameters = connectedNodes.filter(node => node.type === 'parameter_node' || node.type === 'parameter_slider_node' || node.type === 'parameter_slider_bool_node' || node.type === 'parameter_slider_neuronnes_node' || node.type === 'parameter_text_node' || node.type === 'parameter_bool_node');
    
        const getNodeData = (nodeId) => 
        {
            const node = connectedNodes.find(node => node.id === nodeId);
            return node ? node.data : null;
        };
    
        const getConnectedNodesData = (nodeId) => 
        {
            const connectedEdges = this.props.edges.filter(edge => edge.source === nodeId || edge.target === nodeId);
            const connectedNodeIds = connectedEdges.map(edge => edge.source === nodeId ? edge.target : edge.source);
            return connectedNodeIds.map(id => getNodeData(id)).filter(data => data);
        };
    
        const structure = 
        {
            modele: modelNode.data.label,
            fonctions: functions.map(func => (
            {
                label: func.data.label,
                parametres: parameters.filter(param => 
                    this.props.edges.some(edge => 
                        (edge.source === func.id && edge.target === param.id) || 
                        (edge.target === func.id && edge.source === param.id)
                    )
                ).map(param => 
                {
                    if (param.data.label === 'Neuronnes') 
                    {
                        // const connectedData = getConnectedNodesData(param.id).filter(data => data.label !== 'Initialisation');
                        // const neuronnesData = connectedData.map(data => ( data.value || 0 ));

                        const connectedData = getConnectedNodesData(param.id)
                        .filter(data => data.label !== 'Initialisation')
                        .sort((a, b) => a.order - b.order);
                    
                        const coucheSortie = connectedData.find(data => data.label === 'Couche de sortie');
                        const neuronnesData = connectedData.filter(data => data.label !== 'Couche de sortie').map(data => data.value || 0);

                        if (coucheSortie) 
                        {
                            neuronnesData.push(coucheSortie.value || 0);
                        }

                        return {
                            label: 'Neuronnes',
                            value: neuronnesData
                        };
                    }
                    
                    return {
                        label: param.data.label,
                        value: param.data.value || 0
                    };
                })
            }))
        };
    
        return structure;
    }
        

    isConnected = (sourceId, targetId) => 
    {
        const { edges } = this.props;
        return edges.some(edge => (edge.source === sourceId && edge.target === targetId) || (edge.source === targetId && edge.target === sourceId));
    }

    checkIfCompleted = (node) => 
    {
        const { edges } = this.props;
        const targetHandles = node.data.input || [];
        const sourceHandles = node.data.output || [];

        const allTargetHandlesConnected = targetHandles.every(handle => 
            edges.some(edge => edge.targetHandle === handle && edge.target === node.id)
        );

        const allSourceHandlesConnected = sourceHandles.every(handle => 
            edges.some(edge => edge.sourceHandle === handle && edge.source === node.id)
        );

        return allTargetHandlesConnected && allSourceHandlesConnected;
    }

    handleButtonClick = () => 
    {
        const { nodes } = this.props;
        const modelNode = nodes.find(node => node.type === 'modele_node');
        if (modelNode) 
        {
            const connectedNodes = this.getConnectedNodes(modelNode.id);
            const structure = this.generateStructure(connectedNodes);
            console.log(JSON.stringify(structure, null, 2));
        }
    }

    
    

    render()
    {
        const { nodes, edges, onNodesChange, onEdgesChange, onConnect, onNodesDelete, nodeTypes, isValidConnection, proOptions, updateNodeData, updateAllNodeOrders, nodesData } = this.props;
        
        
        return (
            <div className="interface f f-justify-center f-align-center" ref={(ref) => { this.reactFlowWrapper = ref; }} onDrop={this.onDrop} onDragOver={this.onDragOver} style={{ height: 500 }}>
                <ReactFlow nodes={nodes.map(node => ({
                        ...node,
                        data: {
                            ...node.data,
                            nodeLength: nodesData.filter(node => node.data.label == "Couche Cachée").length + 1,
                            updateNodeData: updateNodeData,
                            updateAllNodeOrders: updateAllNodeOrders, 
                            nodesData: nodesData,
                        }
                    }))} edges={edges} onNodesChange={onNodesChange} onEdgesChange={onEdgesChange} onConnect={onConnect} onNodesDelete={onNodesDelete} nodeTypes={nodeTypes} isValidConnection={isValidConnection} proOptions={proOptions} minZoom={0.2}>
                    <Background />
                    <Controls />
                </ReactFlow>
                    <div className='interface-core f f-row f-justify-center f-align-center'>
                        <div className='interface-input f f-row f-justify-center f-align-center' onClick={this.handleButtonClick}>
                            <svg className="interface-svg" width="25" height="25" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" >
                                <path d="M8 5v14l11-7L8 5z" fill="Black" />
                            </svg>
                            <p className='interface-input-text'>Déploiment</p>
                        </div>
                    </div>
            </div>
        )
    }
}

export default Interface;
