import { configureStore  } from 'redux';
import { persistStore, persistReducer } from 'redux-persist';
import storage from 'redux-persist/lib/storage'; // defaults to localStorage for web
import { PersistGate } from 'redux-persist/integration/react';

const initialState = {
  message: 'Hello World',
};

const reducer = (state = initialState, action) => 
{
  switch (action.type) {
    case 'SET_MESSAGE':
      return { ...state, message: action.payload };
    default:
      return state;
  }
};

const persistConfig = {
  key: 'root',
  storage,
};

const persistedReducer = persistReducer(persistConfig, reducer);

const store = configureStore(persistedReducer);
const persistor = persistStore(store);

export { store, persistor };
