import { dashboard } from 'store/reducers/dashboard.reducer'

const DashboardActions = (data) => 
{
    return (dispatch) => { dispatch(dashboard(data)); }
};

export { DashboardActions };