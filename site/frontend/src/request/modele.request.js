import axios from "axios";

class ModeleRequest {
  constructor() {
    this.state = {
      url: 'http://127.0.0.1:8000/api/'
    };
  }

  async requestInitialisation(modele, data) 
  {
    return await axios.post(`${this.state.url}${modele}/`,
      data,
      {
        headers: {
          'Content-Type': 'application/json',
        }
      }
    )
    .then(response => { return response; })
    .catch(error => { throw error; });
  }
}

export default ModeleRequest;
