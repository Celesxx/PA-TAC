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

    render()
    {
        const { nodes, edges, onNodesChange, onEdgesChange, onConnect, onNodesDelete, nodeTypes } = this.props;
        return (
            <div className="interface-core f f-justify-center f-align-center" ref={(ref) => { this.reactFlowWrapper = ref; }} onDrop={this.onDrop} onDragOver={this.onDragOver} style={{ height: 500 }}>
                <ReactFlow nodes={nodes} edges={edges} onNodesChange={onNodesChange} onEdgesChange={onEdgesChange} onConnect={onConnect} onNodesDelete={onNodesDelete} nodeTypes={nodeTypes}>
                    <Background />
                    <Controls />
                </ReactFlow>
            </div>
        )
    }
}

export default Interface;
