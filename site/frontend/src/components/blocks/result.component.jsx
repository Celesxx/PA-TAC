import React from 'react';
import "../../assets/css/blocks/result.asset.css"

class Result extends React.Component 
{

    constructor(props) 
    {
        super(props);
        this.state = 
        { 
            height: 270,
            isResizing: false
        };
        this.resultBar = React.createRef();
        this.resizerRef = React.createRef();
    }

    componentDidMount() 
    {
        document.addEventListener('mousemove', this.handleMouseMove);
        document.addEventListener('mouseup', this.handleMouseUp);
    }

    componentWillUnmount() 
    {
        document.removeEventListener('mousemove', this.handleMouseMove);
        document.removeEventListener('mouseup', this.handleMouseUp);
    }

    handleMouseDown = (e) => { this.setState({ isResizing: true }); };
    handleMouseUp = (e) => { this.setState({ isResizing: false }); };

    handleMouseMove = (e) => 
    {
        if (!this.state.isResizing) return;
        const offsetTop = this.resultBar.current.getBoundingClientRect().bottom;
        const newHeight = offsetTop - e.clientY;
        if (newHeight < 170) { this.setState({ height: 170 }); }
        else if (newHeight > 600) { this.setState({ height: 600 }); }
        else { this.setState({ height: newHeight }); }
    };

    render()
    {
        const { height } = this.state;
        return (
            <div className="result-core f f-justify-center f-align-center" style={{ height: `${height}px`}} ref={this.resultBar}>
                <p>display result here</p>
                <div 
                    className="result-resizer" 
                    onMouseDown={this.handleMouseDown}
                    ref={this.resizerRef}
                ></div>
            </div>
        )
    }
}

export default Result;