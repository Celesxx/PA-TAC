import React from 'react';
import "../../assets/css/blocks/result.asset.css"

class Result extends React.Component 
{

    constructor(props) 
    {
        super(props);
        this.state = { };
    }

    render()
    {
        return (
            <div className="result-core f f-justify-center f-align-center">
                <p>display result here</p>
            </div>
        )
    }
}

export default Result;