import React from 'react';
import ReactFlow, { Controls, Background } from 'reactflow';
import 'reactflow/dist/style.css';
import "../../assets/css/blocks/interface.asset.css"
import InterfaceHelpers from '../../helpers/interface.helper.js';
import ModeleRequest from '../../request/modele.request.js';
class Interface extends React.Component 
{
    constructor(props) 
    {
        super(props);
        this.state = { };
    }


    handleButtonClick = async () => 
    {
        const { nodes, edges, nodesData, updateResult } = this.props;
        const modelNode = nodes.find(node => node.type === 'modele_node');
        if (modelNode) 
        {
            const interfaceHelpers = new InterfaceHelpers();
            const connectedNodes = interfaceHelpers.getConnectedNodes(nodes, edges, nodesData, modelNode.id);
            const structure = interfaceHelpers.generateStructure(connectedNodes, edges);

            console.log(JSON.stringify(structure, null, 2));

            if (structure.fonctions.length === 0) 
            {
                updateResult({id: this.props.result.length, message: "Le modèle n'est pas valide", status: "error"});
                return;
            }

            if (structure.fonctions.length == 4) 
            {
                const modeleRequest = new ModeleRequest();
                const parameters = structure.fonctions.flatMap(func => func.parametres);
                const modeleType = structure.modele === "Perceptron Multi Couche" ? "mlp" : "rbf";

                const result = await modeleRequest.requestInitialisation(modeleType, { 'parametres': parameters });
                updateResult({id: this.props.result.length, message: result.data.message, status: result.status, type: "message"});
                updateResult({id: this.props.result.length, type: "loading"});

            } else {
                updateResult({id: this.props.result.length, message: "Le modèle n'est pas valide", status: "error", type: "message"});
                return
            }
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
