import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/tauri';
import CryptoJS from 'crypto-js';

const secretKey = 'SUA_CHAVE_SECRETA';

const encryptData = (data: any) => {
  return CryptoJS.AES.encrypt(data, secretKey).toString();
};

const decryptData = (ciphertext: any) => {
  const bytes = CryptoJS.AES.decrypt(ciphertext, secretKey);
  return bytes.toString(CryptoJS.enc.Utf8);
};

const Login: React.FC = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const navigate = useNavigate();
  const [showPassword, setShowPassword] = useState(false);
  const [errorMessage, setErrorMessage] = useState('');

  const handleEmailChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setEmail(e.target.value);
  };

  const handlePasswordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setPassword(e.target.value);
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setErrorMessage('');

    try {
      const result = await invoke('authenticate_login', { email, password });
      console.log('Resultado da Autenticação:', result);
      if (result === true) {
        localStorage.setItem('email', encryptData(email));
        localStorage.setItem('password', encryptData(password));
        navigate('/home');
      } else if (typeof result === 'string') {
        console.log('Erro recebido:', result);
        setErrorMessage(result);
      }
    } catch (error) {
      console.error('Erro na autenticação:', error);
      setErrorMessage('Erro na conexão com o servidor.');
    }
  };

  useEffect(() => {
    const savedEmail = localStorage.getItem('email');
    const savedPassword = localStorage.getItem('password');

    if (savedEmail && savedPassword) {
      setEmail(decryptData(savedEmail));
      setPassword(decryptData(savedPassword));
    }
    const checkBackendConnection = async () => {
      try {
        const response = await invoke('test_connection');
        console.log(response);
      } catch (error) {
        console.error('Erro ao testar conexão com o backend:', error);
      }
    };

    checkBackendConnection();
  }, []);

  return (
    <div className='flex flex-col items-center justify-center min-h-screen bg-gradient-to-r from-primary-dark via-primary-light to-primary-dark'>
      <div className='max-w-md w-full px-6 py-8 bg-primary-moreLighter shadow-md rounded-md'>
        <h1 className='text-3xl w-full bg-primary-moreLighter rounded-md text-center text-primary-almostBlack'>
          Vetor - Soluções Inteligentes
        </h1>
        <h2 className='text-2xl font-bold mb-2 text-center mt-4 text-primary-almostBlack'>
          Login
        </h2>
        {errorMessage && (
          <div className='text-red-500 text-center'>{errorMessage}</div>
        )}
        <form onSubmit={handleSubmit}>
          <div className='mb-4'>
            <label
              htmlFor='email'
              className='block mb-2 text-sm font-medium text-primary-dark'
            >
              Email
            </label>
            <input
              type='email'
              id='email'
              className='w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-dark focus:border-primary-dark'
              value={email}
              onChange={handleEmailChange}
              required
            />
          </div>
          <div className='mb-4 relative'>
            <label
              htmlFor='password'
              className='block mb-2 text-sm font-medium text-primary-dark'
            >
              Senha
            </label>
            <input
              type={showPassword ? 'text' : 'password'}
              id='password'
              className='w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-dark focus:border-primary-dark'
              value={password}
              onChange={handlePasswordChange}
              required
            />
            <button
              type='button'
              className='absolute inset-y-0 right-0 pr-3 flex items-center text-xs leading-5 pt-6'
              onClick={() => setShowPassword(!showPassword)}
            >
              {/* Ícone ou texto aqui */}
              {showPassword ? 'Esconder' : 'Mostrar'}
            </button>
          </div>
          <button
            type='submit'
            className='w-full py-2 px-4 text-white bg-secondary rounded-md hover:bg-secondary-light transition-colors'
          >
            Acessar
          </button>
        </form>
        <div className='mt-6 text-center'>
          <span className='text-gray-500 text-xs'>Ou acesse com o Google</span>
        </div>
        <div className='flex justify-center mt-2'>
          <button className='flex items-center justify-center w-10 h-10 bg-red-500 rounded-full text-white mr-2'>
            <img className='h-full' src='/images/google.jpg' alt='Google' />
          </button>
        </div>
      </div>
    </div>
  );
};

export default Login;
