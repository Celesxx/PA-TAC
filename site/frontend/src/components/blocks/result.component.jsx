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
            isResizing: false,
            progress: 0,
            loss: null
        };
        this.resultBar = React.createRef();
        this.resizerRef = React.createRef();
    }

    
    componentDidMount() 
    {
        document.addEventListener('mousemove', this.handleMouseMove);
        document.addEventListener('mouseup', this.handleMouseUp);
        this.connectWebSocket();

    }

    componentWillUnmount() 
    {
        document.removeEventListener('mousemove', this.handleMouseMove);
        document.removeEventListener('mouseup', this.handleMouseUp);
        if (this.socket) {
            console.log("test")
            this.socket.close();
        }
    }

    handleMouseDown = (e) => { this.setState({ isResizing: true }); };
    handleMouseUp = (e) => { this.setState({ isResizing: false }); };

    connectWebSocket = () => 
    {
        this.socket = new WebSocket('ws://127.0.0.1:8000/ws/progress/');
        this.socket.onopen = (event) => 
        {
            console.log('WebSocket connection established');
        };

        this.socket.onmessage = (event) => 
        {
            console.log("test");
            const data = JSON.parse(event.data);
            this.setState({ progress: data.epoch, loss: data.loss });
        };

        this.socket.onerror = (error) => {
            console.error('WebSocket error:', error);
        };

        this.socket.onclose = (event) => {
            console.log("test2")
            console.log('WebSocket closed:', event);
        };
    };

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
        const { height, progress, loss } = this.state;
        const { result } = this.props;
        return (
            <div className="result-core f f-justify-center f-align-center" style={{ height: `${height}px`}} ref={this.resultBar}>
                <div className="result-content f f-column f-align-start f-justify-start">
                    {result.map((res, index) => 
                        (
                            <div key={index} className="result-item f f-row f-align-center f-justify-start">
                                <span className="result-timestamp">{res.timestamp}</span>
                                <span className="result-message">{res.message}</span>
                            </div>
                        )
                    )}
                    {result.some(res => res.type === "loading") && 
                    (
                        <div className="progress-bar">
                            <div className="progress-bar-inner" style={{ width: `${progress}%` }}>
                                <span>{progress}% - Loss: {loss}</span>
                            </div>
                        </div>
                    )}
                </div>
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