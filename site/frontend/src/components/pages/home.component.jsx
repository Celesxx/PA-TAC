import '../../assets/css/global.asset.css';
import '../../assets/css/pages/home.asset.css';
import React from "react";
import Interface from "../../components/blocks/interface.component.jsx"
import Result from "../../components/blocks/result.component.jsx"
import Leftbar from "../../components/blocks/leftbar.component.jsx"
import { addEdge, applyEdgeChanges, applyNodeChanges } from 'reactflow';


class Home extends React.Component 
{

  constructor(props) 
  {
    super(props);
    this.state = 
    {
      nodes: [],
      edges: [],
      nodeIndex: 1
    };
  }

  onNodesChange = (changes) => 
  {
      this.setState({ nodes: applyNodeChanges(changes, this.state.nodes) });
  };
  
  onEdgesChange = (changes) => 
  {
      this.setState({ edges: applyEdgeChanges(changes, this.state.edges) });
  };
  
  onConnect = (params) => 
  {
      this.setState({ edges: addEdge(params, this.state.edges) });
  };
  
  onAddNode = (type) => 
  {
      const { nodes, nodeIndex } = this.state;
      const position = { x: 200, y: nodeIndex * 75 };
      const newNode = 
      {
        id: this.getId(),
        type,
        position,
        data: { label: `${type} node` },
      };
  
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
  
  
  getId = () =>
  {
      if (!this.id) this.id = 0;
      return `modele-box_${this.id++}`;
  };

  render()
  {
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
                />
                <Result/>
              </div>
          </div>
      )
  }
}

export default Home;