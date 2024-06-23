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
                        <h2 className='leftbar-modele-title'>Modèle disponible</h2>
                        {/* <p className='leftbar-modele-desc'>description</p> */}
                        <div className='leftbar-modele-box' ref={ref => this.modelBox = ref}>
                            <div className="modele-box" onClick={() => onAddNode('Perceptron Multi Couche', ["Initialisation", "Entrainement", "Prédiction", "Libération mémoire"], [], "modele_node")}>
                                Perceptron Multi Couche
                            </div>
                            <div className="modele-box" onClick={() => onAddNode('Radial Basis Function', ["Initialisation", "Entrainement", "Prédiction", "Libération mémoire"], [], "modele_node")}>
                                Radial Basis Function
                            </div>
                        </div>
                    </div>

                    <div className='leftbar-modele-core f f-column f-justify-center f-align-start'>
                        <h2 className='leftbar-modele-title'>Fonction du modèle</h2>
                        {/* <p className='leftbar-modele-desc'>description</p> */}
                        <div className='leftbar-modele-box' ref={ref => this.modelBox = ref}>
                            <div className="modele-box" onClick={() => onAddNode('Initialisation', ["Learning rate", "Neuronnes"], ["modèle"], "fonction_node")}>
                                Initialisation
                            </div>
                            <div className="modele-box" onClick={() => onAddNode('Entrainement', ["Epochs", "Batch size", "Type", "Callback", "Log", "Dataset"], ["Modèle"], "fonction_node")}>
                                Entrainement
                            </div>
                            <div className="modele-box" onClick={() => onAddNode('Prédiction', ["Type", "Dataset"], ["modèle"], "fonction_node")}>
                                Prédiction
                            </div>
                            <div className="modele-box" onClick={() => onAddNode('Libération mémoire', [], ["modèle"], "fonction_node")}>
                                Libération mémoire
                            </div>
                        </div>
                    </div>

                    <div className='leftbar-parameter-core f f-column f-justify-center f-align-start'>
                        <h2 className='leftbar-parameter-title'>Choose a parameter</h2>
                        {/* <p className='leftbar-parameter-desc'>description</p> */}
                        <div className='leftbar-modele-box' ref={ref => this.modelBox = ref}>
                            <div className="modele-box" onClick={() => onAddNode('Learning rate', [], ["Initialisation"], "parameter_node")}>
                                Learning rate
                            </div>
                            <div className="modele-box" onClick={() => onAddNode('Epoch', [], ["Entrainement"], "parameter_node")}>
                                Epoch
                            </div>
                            <div className="modele-box" onClick={() => onAddNode('Batch size', [], ["Entrainement"], "parameter_node")}>
                                Batch size
                            </div>
                            <div className="modele-box" onClick={() => onAddNode('Couche du modèle', [], ["Initialisation"], "parameter_node")}>
                                Couche du modèle
                            </div>
                        </div>
                    </div>

                    <div className='leftbar-data-core f f-column f-justify-center f-align-start'>
                        <h2 className='leftbar-data-title'>Choose a data</h2>
                        {/* <p className='leftbar-data-desc'>description</p> */}
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