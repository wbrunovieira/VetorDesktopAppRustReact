import { BrowserRouter, Routes, Route } from 'react-router-dom';
import './styles.css';
import Login from './components/Login';
import Home from './components/Home';
import UserDetails from './components/UserDetails';

const App = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path='/' element={<Login />} />
        <Route path='/login' element={<Login />} />
        <Route path='/home' element={<Home />} />
        <Route path='/usuario/:cpf' element={<UserDetails />} />
      </Routes>
    </BrowserRouter>
  );
};

export default App;
