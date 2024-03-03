import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

const Login: React.FC = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const navigate = useNavigate();

  const handleEmailChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setEmail(e.target.value);
  };

  const handlePasswordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setPassword(e.target.value);
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
  };

  return (
    <div className='flex flex-col items-center justify-center min-h-screen bg-gray-100'>
      <div className='max-w-md w-full px-6 py-8 bg-white shadow-md rounded-md'>
        <h1 className='text-3xl w-full bg-white rounded-md text-center'>
          Vetor - Soluções Intêligentes
        </h1>
        <h2 className='text-2xl font-bold mb-2 text-center mt-4'>Login</h2>
        <form onSubmit={handleSubmit}>
          <div className='mb-4'>
            <label
              htmlFor='email'
              className='block mb-2 text-sm font-medium text-gray-700'
            >
              Email
            </label>
            <input
              type='email'
              id='email'
              className='w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-blue-500'
              value={email}
              onChange={handleEmailChange}
              required
            />
          </div>
          <div className='mb-4'>
            <label
              htmlFor='password'
              className='block mb-2 text-sm font-medium text-gray-700'
            >
              Senha
            </label>
            <input
              type='password'
              id='password'
              className='w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-blue-500'
              value={password}
              onChange={handlePasswordChange}
              required
            />
          </div>
          <button
            type='submit'
            className='w-full py-2 px-4 text-white bg-blue-500 rounded-md hover:bg-blue-600 transition-colors'
            onClick={() => navigate('/Home')}
          >
            Acessar
          </button>
        </form>
        <div className='mt-6 text-center'>
          <span className='text-gray-500'>Ou acesse com o Google</span>
        </div>
        <div className='flex justify-center mt-4'>
          <button className='flex items-center justify-center w-10 h-10 bg-red-500 rounded-full text-white mr-2'>
            <svg
              xmlns='http://www.w3.org/2000/svg'
              fill='none'
              viewBox='0 0 24 24'
              stroke='currentColor'
              className='w-6 h-6'
            >
              <path
                strokeLinecap='round'
                strokeLinejoin='round'
                strokeWidth={2}
                d='M12 6v6m0 0v6m0-6h6m-6 0H6'
              />
            </svg>
          </button>
        </div>
      </div>
    </div>
  );
};

export default Login;
