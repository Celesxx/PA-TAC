import { createSlice, isPlainObject } from '@reduxjs/toolkit'

const initialState= 
{
  videoSrc: [],
  startLoading: false,
  loading: 0,
  loadingMax: 12,
  loadingOver: false,
  resToken: null, 
  resStable: null, 
  totalSupply: null,
  totalBurn: null,
  badges: [],
  tokenUser: 
  {
    balance: null,
    allowanceLm: false,
  },
  stableUser: 
  {
    balance: null,
    allowanceLm: false,
  },
  erc20DispatchManager: {},
  navbarPosition: 200,
}

export const dashboardSlice = createSlice(
{
  name: 'dashboard',
  initialState: 
  {
    videoSrc: [],
    startLoading: false,
    loading: 0,
    loadingMax: 12,
    loadingOver: false,
    resToken: null, 
    resStable: null, 
    totalSupply: null,
    totalBurn: null,
    badges: [],
    tokenUser: 
    {
      balance: null,
      allowanceLm: false,
    },
    stableUser: 
    {
      balance: null,
      allowanceLm: false,
    },
    erc20DispatchManager: {},
    navbarPosition: 200,
  },

  reducers: 
  {
    dashboard: (state, action) => 
    {

      switch(action.payload.action)
      {
        
        case 'saveData':
          for(const [key, value] of Object.entries(action.payload.data))
          {
              if(state[key] !== undefined)
              { 
                if(typeof(value) === "object" && !Array.isArray(value))
                {
                  for(const [key1, value1] of Object.entries(value)) 
                  { 
                    if(state[key][key1] !== undefined)
                    {
                      if(typeof(value1) === "object" && !Array.isArray(value1))
                      {
                        for(const [key2, value2] of Object.entries(value1))
                        {
                          if(state[key][key1][key2] !== undefined) state[key][key1][key2] = value2
                          else state[key][key1] = { ...state[key][key1], ...value1 }
                        }
                      }else
                      {
                        if(state[key][key1] !== undefined) state[key][key1] = value1 
                        else state[key] = {...state[key], ...value}
                        
                      }
                    }else state[key][key1] = value1
                  }
                }else state[key] = value 
              } 
              else console.log(`value not exist : ${key}`)
          }
          break 

        case 'loading':
            state.loading += 1
            if(state.loading == state.loadingMax) { state.loadingOver = true }
            break

        case 'startLoading': 
          state.startLoading = true
          state.loading = 0
          break

        case 'endLoading': 
          state.startLoading = false
          state.loadingOver = false
          break

        case 'navbarPosition': 
          state.navbarPosition = action.payload.navbarPosition
          break
        

        case 'reset': 
          // state = initialState
          for(const [key, value] of Object.entries(initialState)) { if(key !== "navbarPosition") state[key] = value }
          break


        default :
            break
      }
    },
  },
})

export const { dashboard } = dashboardSlice.actions
export default dashboardSlice.reducer