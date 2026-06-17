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
    <div className='flex flex-col items-center justify-center min-h-screen bg-gradient-to-br from-primary-darker via-primary-dark to-primary-light p-6'>
      <div className='max-w-md w-full bg-white shadow-2xl rounded-2xl px-8 py-10'>
        {/* Marca */}
        <div className='flex flex-col items-center'>
          <div className='flex h-14 w-14 items-center justify-center rounded-2xl bg-primary-dark text-3xl font-black text-white shadow-lg'>
            V
          </div>
          <h1 className='mt-4 text-2xl font-bold text-primary-almostBlack'>
            Vetor
          </h1>
          <p className='text-sm text-primary'>Soluções Inteligentes</p>
        </div>

        <h2 className='mt-8 mb-1 text-lg font-semibold text-primary-almostBlack'>
          Acesse sua conta
        </h2>
        <p className='mb-6 text-sm text-gray-500'>
          Entre com suas credenciais para continuar
        </p>

        {errorMessage && (
          <div className='mb-4 flex items-start gap-2 rounded-lg border border-red-200 bg-red-50 px-3 py-2 text-sm text-red-600'>
            <svg className='mt-0.5 h-4 w-4 shrink-0' viewBox='0 0 20 20' fill='currentColor'>
              <path fillRule='evenodd' d='M10 18a8 8 0 100-16 8 8 0 000 16zM8.7 6.3a1 1 0 011.4 0L10 6.6l-.1-.1a1 1 0 011.4 1.4L11.4 8l.1.1a1 1 0 01-1.4 1.4L10 9.4l-.1.1a1 1 0 01-1.4-1.4L8.6 8l-.1-.1a1 1 0 01.2-1.6z' clipRule='evenodd' />
              <path d='M10 13a1 1 0 100 2 1 1 0 000-2z' />
            </svg>
            <span>{errorMessage}</span>
          </div>
        )}

        <form onSubmit={handleSubmit} className='space-y-4'>
          <div>
            <label
              htmlFor='email'
              className='block mb-1.5 text-sm font-medium text-primary-dark'
            >
              Email
            </label>
            <input
              type='email'
              id='email'
              placeholder='voce@empresa.com'
              className='w-full px-3.5 py-2.5 text-primary-almostBlack border border-gray-300 rounded-lg placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-primary-dark focus:border-primary-dark transition'
              value={email}
              onChange={handleEmailChange}
              required
            />
          </div>
          <div>
            <label
              htmlFor='password'
              className='block mb-1.5 text-sm font-medium text-primary-dark'
            >
              Senha
            </label>
            <div className='relative'>
              <input
                type={showPassword ? 'text' : 'password'}
                id='password'
                placeholder='••••••••'
                className='w-full px-3.5 py-2.5 pr-11 text-primary-almostBlack border border-gray-300 rounded-lg placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-primary-dark focus:border-primary-dark transition'
                value={password}
                onChange={handlePasswordChange}
                required
              />
              <button
                type='button'
                aria-label={showPassword ? 'Esconder senha' : 'Mostrar senha'}
                className='absolute inset-y-0 right-0 flex items-center pr-3 text-gray-400 hover:text-primary-dark transition-colors'
                onClick={() => setShowPassword(!showPassword)}
              >
                {showPassword ? (
                  <svg className='h-5 w-5' fill='none' viewBox='0 0 24 24' stroke='currentColor' strokeWidth={1.8}>
                    <path strokeLinecap='round' strokeLinejoin='round' d='M3.98 8.22A10.5 10.5 0 001.93 12C3.23 16.34 7.24 19.5 12 19.5c.99 0 1.95-.14 2.86-.4M6.23 6.23A10.45 10.45 0 0112 4.5c4.76 0 8.77 3.16 10.07 7.5a10.52 10.52 0 01-4.29 5.27M6.23 6.23L3 3m3.23 3.23l3.65 3.65m7.89 7.89L21 21m-3.23-3.23l-3.65-3.65m0 0a3 3 0 10-4.24-4.24m4.24 4.24L9.88 9.88' />
                  </svg>
                ) : (
                  <svg className='h-5 w-5' fill='none' viewBox='0 0 24 24' stroke='currentColor' strokeWidth={1.8}>
                    <path strokeLinecap='round' strokeLinejoin='round' d='M2.04 12.32a1 1 0 010-.64C3.42 7.51 7.36 4.5 12 4.5s8.58 3.01 9.96 7.18a1 1 0 010 .64C20.58 16.49 16.64 19.5 12 19.5s-8.58-3.01-9.96-7.18z' />
                    <path strokeLinecap='round' strokeLinejoin='round' d='M15 12a3 3 0 11-6 0 3 3 0 016 0z' />
                  </svg>
                )}
              </button>
            </div>
          </div>
          <button
            type='submit'
            className='w-full py-2.5 px-4 text-white font-medium bg-secondary rounded-lg shadow-md hover:bg-secondary-light active:scale-[0.99] transition-all'
          >
            Acessar
          </button>
        </form>

        <div className='my-6 flex items-center gap-3'>
          <div className='h-px flex-1 bg-gray-200' />
          <span className='text-xs uppercase tracking-wide text-gray-400'>ou</span>
          <div className='h-px flex-1 bg-gray-200' />
        </div>

        <button
          type='button'
          className='flex w-full items-center justify-center gap-3 rounded-lg border border-gray-300 bg-white py-2.5 px-4 text-sm font-medium text-primary-almostBlack shadow-sm transition-colors hover:bg-gray-50'
        >
          <img className='h-5 w-5 rounded-full object-cover' src='/images/google.jpg' alt='Google' />
          Continuar com Google
        </button>
      </div>

      <p className='mt-6 text-xs text-primary-ligher'>
        © 2026 Vetor · Soluções Inteligentes
      </p>
    </div>
  );
};

export default Login;
