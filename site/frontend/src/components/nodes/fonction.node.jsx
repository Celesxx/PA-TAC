import React from 'react';
import '../../assets/css/nodes/modele.asset.css'; 
import { Handle, Position } from 'reactflow';

const CustomNode = ({ data }) => 
{
  return (
    <div className="node-fonction">
      <div className="node-fonction-header">
        {data.label}
      </div>
      <div className="node-fonction-separator"></div>
      <div className="node-fonction-args f f-row f-align-start f-justify-between f-content-center">
        <div className="node-fonction-args-container f f-column f-align-start f-justify-between f-content-center">
          {Array.isArray(data.output) && data.output.map((arg, index) => (
            <div key={index} className="node-fonction-output f f-row f-align-center f-content-center">
              <div className="node-fonction-text"> {arg} </div>
              <Handle key={index} type="target" position={Position.Left} id={`source-${index}`} className="node-fonction-handle" isConnectable={data.isConnectable} />
            </div>
          ))}
        </div>
        <div className="node-fonction-args-container f f-column f-align-end f-justify-between f-content-center">
          {Array.isArray(data.input) && data.input.map((arg, index) => (
            <div key={index} className="node-fonction-input f f-row f-align-center f-content-center">
              <div className="node-fonction-text"> {arg} </div>
              <Handle key={index} type="source" position={Position.Right} id={`source-${index}`} className="node-fonction-handle" isConnectable={data.isConnectable} />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default CustomNode;
