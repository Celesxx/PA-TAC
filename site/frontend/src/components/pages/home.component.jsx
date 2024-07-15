import '../../assets/css/global.asset.css';
import '../../assets/css/pages/home.asset.css';
import React from "react";
import Interface from "../blocks/interface.component.jsx"
import Result from "../blocks/result.component.jsx"
import Leftbar from "../blocks/leftbar.component.jsx"
import { addEdge, applyEdgeChanges, applyNodeChanges } from 'reactflow';
import ModeleNode from '../nodes/modele.node.jsx';
import ParameterNode from '../nodes/parameter.node.jsx';
import ParameterBoolNode from '../nodes/parameterBool.node.jsx';
import ParameterSliderNode from '../nodes/parameterSlider.node.jsx';
import ParameterSliderNeuronnesNode from '../nodes/parameterSliderNeuronnes.node.jsx';
import ParameterSliderBoolNode from '../nodes/parameterSliderBool.node.jsx';
import ParameterTextNode from '../nodes/parameterText.node.jsx';
import FonctionNode from '../nodes/fonction.node.jsx';
import DatasetNode from '../nodes/dataset.node.jsx';

const nodeTypes = 
{
  modele_node: ModeleNode,
  fonction_node: FonctionNode,
  parameter_node: ParameterNode,
  parameter_bool_node: ParameterBoolNode,
  parameter_slider_node: ParameterSliderNode,
  parameter_slider_neuronnes_node: ParameterSliderNeuronnesNode,
  parameter_text_node: ParameterTextNode,
  parameter_slider_bool_node: ParameterSliderBoolNode,
  dataset_node: DatasetNode,
};

class Home extends React.Component 
{
  constructor(props) 
  {
    super(props);
    this.state = 
    {
      id: 0,
      nodes: [],
      nodesData : [],
      edges: [],
      nodeIndex: 1
    };
  }

  onNodesChange = (changes) => { this.setState({ nodes: applyNodeChanges(changes, this.state.nodes) }); };
  onEdgesChange = (changes) => { this.setState({ edges: applyEdgeChanges(changes, this.state.edges) }); };
  onConnect = (params) => { this.setState({ edges: addEdge({ ...params, animated: true, className: "nodes-edges", type : "smoothstep" }, this.state.edges) }); };
  
  updateNodeData = (nodeId, values) => 
  {
      this.setState(prevState => 
      {
          const existingNodeDataIndex = prevState.nodesData.findIndex(node => node.node === nodeId);
          if (existingNodeDataIndex >= 0) 
          {
            const updatedNodesData = [...prevState.nodesData];
            updatedNodesData[existingNodeDataIndex].data = { ...updatedNodesData[existingNodeDataIndex].data, ...values };
            return { nodesData: updatedNodesData };
          } 
          else 
          {
              return { nodesData: [...prevState.nodesData, { node: nodeId, data: values }] };
          }
      });
  };

  isValidConnection = (connection) => 
  {
    const targetNode = this.state.nodes.find(node => node.id === connection.target);
    if (!targetNode) return false;

    const sourceNode = this.state.nodes.find(node => node.id === connection.source);
    if (!sourceNode) return false;

    return connection.sourceHandle === targetNode.data.label;
  };
  
  onAddNode = (name, label, input, output, type) => 
  {
      const { nodes, nodeIndex } = this.state;
      const position = { x: 200, y: nodeIndex * 75 };
      const newNode = 
      {
        id: this.getId(),
        position,
        data: { label: `${name}`, name: `${label}`, input: input, output: output, isConnectable: true},
        className: 'modele-node',
      };

      if(type != "") { newNode.type = type; }

      this.setState(
      {
        nodes: [...nodes, newNode],
        nodeIndex: nodeIndex + 1
      });
  };

  onNodesDelete = (nodesToDelete) => 
  {
    this.setState((state) => (
    {
      nodes: state.nodes.filter(node => !nodesToDelete.includes(node.id)),
      edges: state.edges.filter(edge => !nodesToDelete.includes(edge.source) && !nodesToDelete.includes(edge.target))
    }));
  };
  
  updateAllNodeOrders = (nodeId, newOrder) => 
  {
    const node = this.state.nodesData.find(node => node.node === nodeId);
    let currentOrder = node.data.order || 0;
    if (currentOrder === newOrder) 
    {
      currentOrder = (currentOrder + 1) % (this.state.nodesData.filter(node => node.data.label === "Couche Cachée").length + 1) != 0 ? currentOrder + 1 : 1;
      this.setState((prevState) => 
      {
        const maxOrder = prevState.nodesData.filter(node => node.data.label === "Couche Cachée").length;
        const updatedNodesData = prevState.nodesData.map(nodeData => 
        {
          if (nodeData.data.label !== "Couche Cachée") { return nodeData; }
          if (nodeData.node === node.node) 
          {
            return { ...nodeData, data: { ...nodeData.data, order: currentOrder } };
          } else
          {
            return { ...nodeData, data: { ...nodeData.data, order: (nodeData.data.order + 1) % (maxOrder + 1) != 0 ? nodeData.data.order + 1 : 1} };
          }
        });
        return { nodesData: updatedNodesData };
      });
    }
  };
  

  
  getId = () =>
  {
      if (!this.id) this.id = 0;
      return `modele-box_${this.id++}`;
  };

  render()
  {
    const proOptions = { hideAttribution: true };
      return(
          <div className="home f f-row f-align-center f-justify-between">
              <Leftbar onAddNode={this.onAddNode} />
              <div className='home-core f f-column f-align-center f-justify-around'>
                <Interface 
                  nodes={this.state.nodes} 
                  edges={this.state.edges} 
                  onNodesChange={this.onNodesChange} 
                  onEdgesChange={this.onEdgesChange} 
                  onConnect={this.onConnect} 
                  onNodesDelete={this.onNodesDelete}
                  nodeTypes={nodeTypes}
                  isValidConnection={this.isValidConnection}
                  proOptions={proOptions}
                  updateNodeData={this.updateNodeData}
                  nodesData={this.state.nodesData}
                  updateAllNodeOrders={this.updateAllNodeOrders}
                />
                <Result/>
              </div>
          </div>
      )
  }
}

export default Home;
