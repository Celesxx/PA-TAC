import React from 'react';
import "../../assets/css/blocks/leftbar.asset.css"
import dataModele from "../../data/modele.data.json"

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
                    <h1 className='leftbar-title'>Deep Learnia</h1>
                </div>


                <div className='leftbar-core f f-column f-justify-around f-align-center'>

                    <div className='leftbar-modele-core f f-column f-justify-center f-align-start'>
                        <h2 className='leftbar-modele-title'>Liste des modèles</h2>
                        <div className='leftbar-modele-box' ref={ref => this.modelBox = ref}>
                            {dataModele["modeles"].map((modele, index) => (
                                <div 
                                    key={index} 
                                    className="modele-box" 
                                    onClick={() => onAddNode(modele.name, modele.input, modele.output, modele.type)}
                                >
                                    {modele.name}
                                </div>
                            ))}
                        </div>
                    </div>

                    <div className='leftbar-modele-core f f-column f-justify-center f-align-start'>
                        <h2 className='leftbar-modele-title'>Liste des méthodes</h2>
                        <div className='leftbar-modele-box leftbar-methode-box' ref={ref => this.modelBox = ref}>
                            {dataModele["methodes"].map((modele, index) => (
                                <div 
                                    key={index} 
                                    className="methode-box" 
                                    onClick={() => onAddNode(modele.name, modele.input, modele.output, modele.type)}
                                >
                                    {modele.name}
                                </div>
                            ))}
                        </div>
                    </div>

                    <div className='leftbar-parameter-core f f-column f-justify-center f-align-start'>
                        <h2 className='leftbar-modele-title'>Liste des paramètres</h2>
                        <div className='leftbar-modele-box leftbar-parametre-box' ref={ref => this.modelBox = ref}>
                            {dataModele["parametres"].map((modele, index) => (
                                <div 
                                    key={index} 
                                    className="parametre-box" 
                                    onClick={() => onAddNode(modele.name, modele.input, modele.output, modele.type)}
                                >
                                    {modele.name}
                                </div>
                            ))}
                        </div>
                    </div>

                    <div className='leftbar-data-core f f-column f-justify-center f-align-start'>
                        <h2 className='leftbar-modele-title'>Liste des datasets</h2>
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