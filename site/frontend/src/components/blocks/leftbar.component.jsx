import React from 'react';
import "../../assets/css/blocks/leftbar.asset.css"


class Leftbar extends React.Component 
{

    constructor(props) 
    {
        super(props);
        this.state = 
        { 
            
        };
    }

    scrollLeft = () => { this.modelBox.scrollBy({ left: -150, behavior: 'smooth' }); };

    scrollRight = () => { this.modelBox.scrollBy({ left: 150, behavior: 'smooth' }); };
  
    render()
    {
        const { onAddNode } = this.props;
        return (
            <div className="leftbar f f-column f-justify-start f-align-center">

                <div className='leftbar-title-core f f-column f-justify-start f-align-center'>
                    <h1 className='leftbar-title'>Title</h1>
                    <p className='leftbar-desc'>description</p>
                </div>


                <div className='leftbar-core f f-column f-justify-around f-align-center'>

                    <div className='leftbar-modele-core f f-column f-justify-center f-align-start'>
                        <h2 className='leftbar-modele-title'>Choose a model</h2>
                        <p className='leftbar-modele-desc'>description</p>
                        <div className='leftbar-modele-box' ref={ref => this.modelBox = ref}>
                            <div className="modele-box" onClick={() => onAddNode('Perceptron Multi Couche', ["Neuronnes", "Learning rate", "Batch Size", "Epoch"])}>
                                Perceptron Multi Couche
                            </div>
                            <div className="modele-box" onClick={() => onAddNode('Radial Basis Function', ["Neuronnes", "Learning rate", "Epoch"])}>
                                Radial Basis Function
                            </div>
                        </div>
                    </div>

                    <div className='leftbar-parameter-core f f-column f-justify-center f-align-start'>
                        <h2 className='leftbar-parameter-title'>Choose a parameter</h2>
                        <p className='leftbar-parameter-desc'>description</p>
                        <div className='leftbar-parameter-box'>
                            
                        </div>
                    </div>

                    <div className='leftbar-data-core f f-column f-justify-center f-align-start'>
                        <h2 className='leftbar-data-title'>Choose a data</h2>
                        <p className='leftbar-data-desc'>description</p>
                        <div className='leftbar-data-box'>
                            
                        </div>
                    </div>

                </div>

                <div className='leftbar-bottom-core'>
                    
                </div>
                
            </div>
        )
    }
}

export default Leftbar;