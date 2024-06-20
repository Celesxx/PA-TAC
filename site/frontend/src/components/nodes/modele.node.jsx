import React from 'react';
import '../../assets/css/nodes/modele.asset.css'; 
import { Handle, Position } from 'reactflow';

const CustomNode = ({ data, isConnectable  }) => 
{
  return (
    <div className="node-modele">
      <div className="node-modele-handles">
        {Array.isArray(data.args) && data.args.map((_, index) => (
          <Handle
            key={index}
            type="source"
            position={Position.Left}
            id={`source-${index}`}
            className="node-modele-handle"
            isConnectable={isConnectable}
          />
        ))}
      </div>
      <div className="node-modele-header">
        {data.label}
      </div>
      <div className="node-modele-separator"></div>
      <div className="node-modele-args f f-wrap f-justify-between">
        {Array.isArray(data.args) ? (
          data.args.map((arg, index) => (
            <div key={index} className="node-modele-arg">
              - {arg}
            </div>
          ))
        ) : (
          data.args
        )}
      </div>
    </div>
  );
};

export default CustomNode;
