import React, { useState } from 'react';
import '../../assets/css/nodes/modele.asset.css'; 
import { Handle, Position } from 'reactflow';

const ParameterBoolNode = ({ data }) => 
{

  const [parameterValue, setParameterValue] = useState(true);
  const toggleParameter = () => { setParameterValue(prevValue => !prevValue); };

  return (
    <div className="node-parameter">
      <div className="node-parameter-header">
        {data.label}
      </div>
      <div className="node-parameter-args f f-row f-align-end f-justify-between f-content-center">
        <div className="node-parameter-args-container f f-column f-align-start f-justify-between f-content-center">
          {Array.isArray(data.output) && data.output.map((arg, index) => (
            <div key={index} className="node-parameter-output f f-row f-align-center f-content-center">
              <div className="node-parameter-text"> {arg} </div>
              <Handle key={index} type="target" position={Position.Left} id={arg} className="node-parameter-handle" isConnectable={data.isConnectable} />
            </div>
          ))}
        </div>
        <div className="node-parameter-args-container f f-column f-align-end f-justify-between f-content-center">
          {Array.isArray(data.input) && data.input.map((arg, index) => (
            <div key={index} className="node-parameter-input f f-row f-align-center f-content-center">
              <div className="node-parameter-text"> {arg} </div>
              <Handle key={index} type="source" position={Position.Right} id={arg} className="node-parameter-handle" isConnectable={data.isConnectable} />
            </div>
          ))}
        </div>
      </div>
      <div className="node-parameter-button f f-row f-align-center f-justify-center">
        <button onClick={toggleParameter}>
          {parameterValue ? 'True' : 'False'}
        </button>
      </div>
    </div>
  );
};

export default ParameterBoolNode;
