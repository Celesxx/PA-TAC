import axios from "axios";

class ModeleRequest
{
  async requestInitialisation(modele, data)
  {
    console.log('requestInitialisation', modele, data);
      // return await axios(`${this.state.url}/${modele}`, 
      // {
      //     params: data.parametres
      // })
      // .then(response => response.data)
      // .catch(error => { throw error; });
  }
}

export default ModeleRequest;