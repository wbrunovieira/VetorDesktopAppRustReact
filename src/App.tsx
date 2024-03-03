import { useState } from 'react';

import { invoke } from '@tauri-apps/api/tauri';
import './styles.css';

function App() {
  const [greetMsg, setGreetMsg] = useState('');
  const [name, setName] = useState('');

  async function greet() {
    setGreetMsg(await invoke('greet', { name }));
  }

  return (
    <div className=' bg-black'>
      <h1 className='text-lg text-red-500'>
        Bem vindo ao Sistema IR da Vetor Soluções Intêligentes.{' '}
      </h1>

      <p>entre com o email e senha</p>

      <form
        className='login'
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id='email'
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder='Digite o email'
        />
        <input
          id='password'
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder='Digite a senha'
        />
        <button type='submit'>Enviar</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
