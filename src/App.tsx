import { useState } from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import './styles.css';
import Login from './components/Login';
import Home from './components/Home';

const App = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path='/Login' element={<Login />} />
        <Route path='/home' element={<Home />} />
      </Routes>
    </BrowserRouter>
  );
};

export default App;
