import { login } from 'store/reducers/login.reducer.js'


const LoginActions = (data) => 
{
    return (dispatch) => { dispatch(login(data)); }
};

export { LoginActions };