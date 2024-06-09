import { createSlice } from '@reduxjs/toolkit'

export const loginSlice = createSlice(
{
  name: 'loginMetamask',
  initialState: {
    address: "",
    activateListener: false,
    language: "en",
  },

  reducers: 
  {
    login: (state, action) => 
    {
      switch(action.payload.action)
      {
          case 'address': 
              state.address = action.payload.address 
              break;

          case 'language':
              state.language = action.payload.language
              break;
              
          case 'activateListener':
              state.activateListener = action.payload.activateListener
              break;

          default: 
            break;
      }
    },
  },
})

export const { login, disconnect } = loginSlice.actions

export default loginSlice.reducer