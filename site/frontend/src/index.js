import React from 'react';
import ReactDOM from 'react-dom/client';
import './assets/css/global.asset.css';
import Home from './components/pages/home.component';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    <Home />
  </React.StrictMode>
);

