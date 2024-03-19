import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

import { Link, useNavigate } from 'react-router-dom';

interface User {
  nome: string;
  cpf: string;
  grupo?: string;
  ativo?: boolean;
  statusFinanceiro?: string;
}

const Home: React.FC = () => {
  const [ano, setAno] = useState('');
  const [nome, setNome] = useState('');
  const [cpf, setCpf] = useState('');
  const [users, setUsers] = useState<User[]>([]);
  const [error, setError] = useState('');
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const users: User[] = await invoke('get_users');
        console.log(users);
        setUsers(users);
      } catch (error) {
        console.error('Erro ao buscar usuários:', error);
        setError('Falha ao carregar usuários.');
      } finally {
        setIsLoading(false);
      }
    };

    fetchData();
  }, []);

  const navigate = useNavigate();

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
  };

  if (error) {
    return <div className='error'>{error}</div>;
  }

  if (isLoading) {
    return <div>Carregando...</div>;
  }

  return (
    <div className='flex flex-col items-center justify-center min-h-screen bg-gradient-to-r from-primary-dark via-primary-light to-primary-dark'>
      <div className='max-w-4xl w-full px-6 py-8 bg-primary-moreLighter shadow-md rounded-md h-[70%]'>
        <div className='flex '>
          <h1 className='text-3xl text-center text-primary-almostBlack mb-8 flex-1'>
            Tela Inicial
          </h1>
          <button
            type='submit'
            className=' px-2 text-white bg-secondary rounded-md hover:bg-secondary-light transition-colors h-6 text-xs shadow-lg'
            onClick={() => navigate('/Login')}
          >
            sair
          </button>
        </div>
        <form
          onSubmit={handleSearch}
          className='mb-4 flex justify-between items-center'
        >
          <input
            type='text'
            placeholder='Ano'
            value={ano}
            onChange={(e) => setAno(e.target.value)}
            className='w-1/4 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-dark focus:border-primary-dark'
          />
          <input
            type='text'
            placeholder='Nome'
            value={nome}
            onChange={(e) => setNome(e.target.value)}
            className='w-1/4 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-dark focus:border-primary-dark'
          />
          <input
            type='text'
            placeholder='CPF'
            value={cpf}
            onChange={(e) => setCpf(e.target.value)}
            className='w-1/4 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-dark focus:border-primary-dark'
          />
          <button
            type='submit'
            className='py-2 px-4 text-white bg-secondary rounded-md hover:bg-secondary-light transition-colors shadow-2xl'
          >
            Pesquisar
          </button>
        </form>
        <div className='overflow-x-auto relative shadow-md sm:rounded-lg'>
          <table className='w-full text-sm text-left text-gray-500'>
            <thead className='text-xs text-primary-moreLighter uppercase bg-primary'>
              <tr>
                <th scope='col' className='py-3 px-6'>
                  Nome
                </th>
                <th scope='col' className='py-3 px-6'>
                  CPF
                </th>
                <th scope='col' className='py-3 px-6'>
                  Grupo
                </th>
                <th scope='col' className='py-3 px-6'>
                  Ativo
                </th>
                <th scope='col' className='py-3 px-6'>
                  Estatus Financeiro
                </th>
              </tr>
            </thead>
            <tbody>
              {users
                ? users.map((usuario, index) => (
                    <tr key={index} className='bg-primary-light border-b'>
                      <th
                        scope='row'
                        className='py-4 px-6 font-medium text-gray-900 whitespace-nowrap'
                      >
                        {usuario.nome}
                      </th>
                      <td className='py-4 px-6'>{usuario.cpf}</td>
                      <td className='py-4 px-6'>{usuario.grupo}</td>
                      <td className='py-4 px-6'>
                        {usuario.ativo ? 'Sim' : 'Não'}
                      </td>
                      <td className='py-4 px-6'>{usuario.statusFinanceiro}</td>
                    </tr>
                  ))
                : null}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};

export default Home;
