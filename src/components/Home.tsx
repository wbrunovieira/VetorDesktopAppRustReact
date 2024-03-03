import { useNavigate } from 'react-router-dom';

const Home = () => {
  const navigate = useNavigate();
  return (
    <div>
      Home
      <button
        type='submit'
        className='w-full py-2 px-4 text-white bg-blue-500 rounded-md hover:bg-blue-600 transition-colors'
        onClick={() => navigate('/Login')}
      >
        Login
      </button>
    </div>
  );
};

export default Home;
