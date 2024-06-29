import React, { useState } from 'react';
import '../../assets/css/nodes/modele.asset.css'; 
import { Handle, Position } from 'reactflow';

const ParameterTextNode = ({ data }) => 
{

  const [inputValue, setInputValue] = useState(1);

  const handleInputChange = (event) => 
  {
    const value = event.target.value;
    if (value === '' || (Number(value) >= 0 && !isNaN(Number(value)))) { setInputValue(value); }
  };

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
      <div className="node-textbox-input-container f f-column f-align-center f-justify-center">
        <input 
          type="number" 
          min="0" 
          value={inputValue} 
          onChange={handleInputChange} 
          className="textbox"
        />
      </div>
    </div>
  );
};

export default ParameterTextNode;
