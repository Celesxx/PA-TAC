import React, { useState, useEffect } from 'react';
import '../../assets/css/nodes/modele.asset.css'; 
import { Handle, Position } from 'reactflow';

const ParameterSliderNode = ({ id, data }) => 
{

  const [sliderValue, setSliderValue] = useState(0);
  const [order, setOrder] = useState(data.nodeLength || 1);

  useEffect(() => 
  {
    data.updateNodeData(id, { 'label': data.label, 'value': sliderValue, "order" : data.nodeLength });
  }, [sliderValue, id]);

  useEffect(() => 
  {
    let filteredNodesData = data.nodesData.find(nodeData => nodeData.node === id);
    if (filteredNodesData) { setOrder(filteredNodesData.data.order); }
  }, [data.nodesData, id]);
  
  const handleOrderChange = () => 
  {
    data.updateAllNodeOrders(id, order);
    let filteredNodesData = data.nodesData.find(nodeData => nodeData.node === id)
    if (filteredNodesData) 
    {
      setOrder(filteredNodesData.data.order);
    }
  };

  const handleSliderChange = (event) => 
  {
    const newValue = parseFloat(event.target.value);
    setSliderValue(roundValue(newValue, 1));
    data.updateNodeData(id, {'label': data.label, 'value': roundValue(newValue, 1)});
  };


  const roundValue = (value, step) => 
  {
    const precision = Math.min(Math.max(Math.log10(1 / step), 0), 100);
    return parseFloat(value.toFixed(precision));
  };

  const incrementValue = () => 
    {
      setSliderValue(prevValue => 
      {
        const newValue = roundValue(prevValue + 1, 1);
        data.updateNodeData(id, { 'label': data.label, 'value': newValue });
        return newValue;
      });
    };
  
    const decrementValue = () => 
    {
      setSliderValue(prevValue => 
      {
        const newValue = roundValue(prevValue - 1, 1);
        data.updateNodeData(id, { 'label': data.label, 'value': newValue });
        return newValue;
      });
    };

  return (
    <div className="node-parameter">
      <div className="node-parameter-header node-neuronnes f f-rows f-justify-between">
        {data.label}
        {
          data.label === "Couche Cachée" &&
          (
            <button className="node-parameter-order" onClick={handleOrderChange}>Order: {order}</button>
          )
        }
        {/* <button className="node-parameter-order" onClick={handleOrderChange}>Order: {order}</button> */}
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
      <div className="node-parameter-slider f f-column f-align-center f-justify-center">
        <div className="slider-control f f-row f-align-center">
          <button className="slider-button" onClick={decrementValue}>-</button>
          <div className="slider-value">Value: {roundValue(sliderValue, 1)}</div>
          <button className="slider-button" onClick={incrementValue}>+</button>
        </div>
        <input 
          type="range" 
          min={0} 
          max={250}
          step={1}
          value={sliderValue} 
          onChange={handleSliderChange}
          className="slider nodrag"
        />
      </div>
    </div>
  );
};

export default ParameterSliderNode;
