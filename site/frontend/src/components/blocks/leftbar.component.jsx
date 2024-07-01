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
            width: 270,
            isResizing: false
        };
        this.leftbarRef = React.createRef();
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
    scrollLeft = () => { this.modelBox.scrollBy({ left: -150, behavior: 'smooth' }); };
    scrollRight = () => { this.modelBox.scrollBy({ left: 150, behavior: 'smooth' }); };

    handleMouseMove = (e) => 
    {
        if (!this.state.isResizing) return;
        let newWidth = e.clientX - this.leftbarRef.current.getBoundingClientRect().left;
        if (newWidth < 270) { newWidth = 270; }
        else if (newWidth > 500) { newWidth = 500; }
        this.setState({ width: newWidth });
    };

    render()
    {
        const { onAddNode } = this.props;
        const { width } = this.state;
        return (
            <div className="leftbar f f-column f-justify-start f-align-center" style={{ width: `${width}px` }} ref={this.leftbarRef}>

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
                                    onClick={() => onAddNode(modele.name, "", modele.input, modele.output, modele.type)}
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
                                    onClick={() => onAddNode(modele.name, "", modele.input, modele.output, modele.type)}
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
                                    onClick={() => onAddNode(modele.name, "",  modele.input, modele.output, modele.type)}
                                >
                                    {modele.name}
                                </div>
                            ))}
                        </div>
                    </div>

                    <div className='leftbar-data-core f f-column f-justify-center f-align-start'>
                        <h2 className='leftbar-modele-title'>Liste des datasets</h2>
                        <div className='leftbar-modele-box leftbar-dataset-box' ref={ref => this.modelBox = ref}>
                            {dataModele["dataset"].map((modele, index) => (
                                <div 
                                    key={index} 
                                    className="dataset-box" 
                                    onClick={() => onAddNode(modele.name, modele.label,  modele.input, modele.output, modele.type)}
                                >
                                    {modele.label}
                                </div>
                            ))}
                        </div>
                    </div>

                </div>

                <div className='leftbar-bottom-core'>
                    
                </div>

                <div 
                    className="resizer" 
                    onMouseDown={this.handleMouseDown}
                    ref={this.resizerRef}
                ></div>

            </div>
        )
    }
}

export default Leftbar;