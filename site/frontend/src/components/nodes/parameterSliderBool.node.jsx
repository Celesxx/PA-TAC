import React, { useState, useEffect  } from 'react';
import '../../assets/css/nodes/modele.asset.css'; 
import { Handle, Position } from 'reactflow';

const ParameterSliderBoolNode = ({ id, data }) => 
{

  const [sliderValue, setSliderValue] = useState(0);
  const [stepExponent, setStepExponent] = useState(0);
  const [step, setStep] = useState(Math.pow(10, stepExponent));
  const [parameterValue, setParameterValue] = useState(true);
  
  const toggleParameter = () => 
  { 

    setParameterValue(prevValue => 
    {
      const newValue = !prevValue;
      data.updateNodeData(id, { 'label': data.label, 'value': sliderValue, 'enable': newValue });
      return newValue;
    });
  };

  useEffect(() => 
  {
    const newStep = Math.pow(10, stepExponent);
    setStep(newStep);
    const newMin = newStep;
    const newMax = newStep * 9;
    setSliderValue(Math.max(newMin, Math.min(newMax, sliderValue)));
  }, [stepExponent]);

  useEffect(() => 
  {
    data.updateNodeData(id, { 'label': data.label, 'value': sliderValue, 'enable': parameterValue });
  }, []);

  const handleSliderChange = (event) => 
  {
    const newValue = parseFloat(event.target.value);
    setSliderValue(roundValue(newValue, step));
    data.updateNodeData(id, {'label': data.label, 'value': roundValue(newValue, step), 'enable': parameterValue});
  };

  const handleStepExponentChange = (event) => 
  {
    const newStepExponent = parseInt(event.target.value, 10);
    setStepExponent(newStepExponent);
  };


  const roundValue = (value, step) => 
  {
    const precision = Math.min(Math.max(Math.log10(1 / step), 0), 100);
    return parseFloat(value.toFixed(precision));
  };

  const calculateMinMax = (step) => 
  {
    const newMin = step;
    const newMax = step * 9;
    return { min: newMin, max: newMax };
  };

  const { min, max } = calculateMinMax(step);
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
      <div className="node-parameter-slider f f-column f-align-center f-justify-center">
        <div className="slider-value">Value: {roundValue(sliderValue, step)}</div>
        <input 
          type="range" 
          min={min} 
          max={max}
          step={step}
          value={sliderValue} 
          onChange={handleSliderChange}
          className="slider nodrag"
        />
      </div>
      <div className="node-step-slider f f-column f-align-center f-justify-center">
        <div className="step-value">Step: 10^ + {stepExponent}</div>
        <input 
          type="range" 
          id="step-slider"
          min={-10} 
          max={10}
          step={1}
          value={stepExponent} 
          onChange={handleStepExponentChange}
          className="slider nodrag"
        />
      </div>
    </div>
  );
};

export default ParameterSliderBoolNode;
