import React from 'react';
import '../../assets/css/nodes/modele.asset.css'; 
import { Handle, Position } from 'reactflow';

const ModeleNode = ({ data }) => 
{
  return (
    <div className="node-modele">
      <div className="node-modele-header">
        {data.label}
      </div>
      <div className="node-modele-args f f-row f-align-start f-justify-between f-content-center">
        <div className="node-modele-args-container f f-column f-align-start f-justify-between f-content-center">
          {Array.isArray(data.output) && data.output.map((arg, index) => (
            <div key={index} className="node-modele-output f f-row f-align-center f-content-center">
              <div className="node-modele-text"> {arg} </div>
              <Handle key={index} type="target" position={Position.Left} id={arg} className="node-modele-handle" isConnectable={data.isConnectable} />
            </div>
          ))}
        </div>
        <div className="node-modele-args-container f f-column f-align-end f-justify-between f-content-center">
          {Array.isArray(data.input) && data.input.map((arg, index) => (
            <div key={index} className="node-modele-input f f-row f-align-center f-content-center">
              <div className="node-modele-text"> {arg} </div>
              <Handle key={index} type="source" position={Position.Right} id={arg} className="node-modele-handle" isConnectable={data.isConnectable} />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default ModeleNode;
