import React from 'react';
import '../../assets/css/nodes/modele.asset.css'; 
import { Handle, Position } from 'reactflow';

const ParameterNode = ({ data }) => 
{
  return (
    <div className="node-dataset">
      <div className="node-dataset-header">
        {data.name}
      </div>
      <div className="node-dataset-args f f-row f-align-end f-justify-between f-content-center">
        <div className="node-dataset-args-container f f-column f-align-start f-justify-between f-content-center">
          {Array.isArray(data.output) && data.output.map((arg, index) => (
            <div key={index} className="node-dataset-output f f-row f-align-center f-content-center">
              <div className="node-dataset-text"> {arg} </div>
              <Handle key={index} type="target" position={Position.Left} id={arg} className="node-dataset-handle" isConnectable={data.isConnectable} />
            </div>
          ))}
        </div>
        <div className="node-dataset-args-container f f-column f-align-end f-justify-between f-content-center">
          {Array.isArray(data.input) && data.input.map((arg, index) => (
            <div key={index} className="node-dataset-input f f-row f-align-center f-content-center">
              <div className="node-dataset-text"> {arg} </div>
              <Handle key={index} type="source" position={Position.Right} id={arg} className="node-dataset-handle" isConnectable={data.isConnectable} />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default ParameterNode;
