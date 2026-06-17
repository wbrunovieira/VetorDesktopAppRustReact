import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

import { useNavigate } from 'react-router-dom';

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
  const [originalUsers, setOriginalUsers] = useState<User[]>([]);

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();

    const filteredUsers = users.filter((user) => {
      return (
        user.nome.toLowerCase().includes(nome.toLowerCase()) &&
        user.cpf.includes(cpf)
      );
    });

    setUsers(filteredUsers);
  };

  const filterUsers = (nome: string, cpf: string) => {
    const filtered = originalUsers.filter(
      (user) =>
        user.nome.toLowerCase().includes(nome.toLowerCase()) &&
        user.cpf.includes(cpf)
    );
    setUsers(filtered);
  };

  const handleNomeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const novoNome = e.target.value;
    setNome(novoNome);
    filterUsers(novoNome, cpf);
  };
  const handleAnoChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const novoAno = e.target.value;
    setAno(novoAno);
    filterUsers(novoAno, ano);
  };

  const handleCpfChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const novoCpf = e.target.value;
    setCpf(novoCpf);
    filterUsers(nome, novoCpf);
  };

  useEffect(() => {
    const fetchData = async () => {
      try {
        const fetched: User[] = await invoke('get_users');
        const grupos = ['Premium', 'Empresarial', 'Padrão'];
        const status = ['Em dia', 'Pendente', 'Atrasado'];
        const enriched = fetched.map((u, i) => ({
          ...u,
          grupo: u.grupo ?? grupos[i % grupos.length],
          ativo: u.ativo ?? i % 4 !== 0,
          statusFinanceiro: u.statusFinanceiro ?? status[i % status.length],
        }));
        setUsers(enriched);
        setOriginalUsers(enriched);
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
            onClick={() => navigate('/')}
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
            onChange={handleAnoChange}
            className='w-1/4 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-dark focus:border-primary-dark'
          />
          <input
            type='text'
            placeholder='Nome'
            value={nome}
            onChange={handleNomeChange}
            className='w-1/4 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-dark focus:border-primary-dark'
          />
          <input
            type='text'
            placeholder='CPF'
            value={cpf}
            onChange={handleCpfChange}
            className='w-1/4 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-dark focus:border-primary-dark'
          />
          <button
            type='submit'
            className='py-2 px-4 text-white bg-secondary rounded-md hover:bg-secondary-light transition-colors shadow-2xl'
          >
            Pesquisar
          </button>
        </form>
        <div className='overflow-x-auto relative shadow-md rounded-lg border border-primary-ligher'>
          <table className='w-full text-sm text-left table-fixed border-collapse'>
            <thead className='text-xs font-semibold text-white uppercase tracking-wider bg-primary-dark'>
              <tr>
                <th scope='col' className='py-3.5 px-6 text-left w-1/5'>
                  Nome
                </th>
                <th scope='col' className='py-3.5 px-6 text-left w-1/5'>
                  CPF
                </th>
                <th scope='col' className='py-3.5 px-6 text-left w-1/5'>
                  Grupo
                </th>
                <th scope='col' className='py-3.5 px-6 text-left w-1/5'>
                  Ativo
                </th>
                <th scope='col' className='py-3.5 px-6 text-left w-1/5'>
                  Status Financeiro
                </th>
              </tr>
            </thead>
            <tbody>
              {users.map((usuario, index) => {
                const statusStyle: Record<string, string> = {
                  'Em dia': 'bg-green-100 text-green-700 ring-green-600/20',
                  Pendente: 'bg-amber-100 text-amber-700 ring-amber-600/20',
                  Atrasado: 'bg-red-100 text-red-700 ring-red-600/20',
                };
                return (
                  <tr
                    key={index}
                    onClick={() => navigate(`/usuario/${usuario.cpf}`)}
                    className={`border-b border-primary-ligher cursor-pointer transition-colors hover:bg-primary-ligher/60 ${
                      index % 2 === 0 ? 'bg-white' : 'bg-primary-moreLighter'
                    }`}
                  >
                    <td className='py-3.5 px-6 text-left font-medium text-primary-almostBlack'>
                      {usuario.nome}
                    </td>
                    <td className='py-3.5 px-6 text-left text-primary-darker tabular-nums'>
                      {usuario.cpf}
                    </td>
                    <td className='py-3.5 px-6 text-left'>
                      <span className='inline-flex items-center rounded-md bg-primary-ligher px-2 py-0.5 text-xs font-medium text-primary-darker'>
                        {usuario.grupo}
                      </span>
                    </td>
                    <td className='py-3.5 px-6 text-left'>
                      <span
                        className={`inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-medium ${
                          usuario.ativo
                            ? 'bg-green-100 text-green-700'
                            : 'bg-gray-200 text-gray-500'
                        }`}
                      >
                        <span
                          className={`h-1.5 w-1.5 rounded-full ${
                            usuario.ativo ? 'bg-green-500' : 'bg-gray-400'
                          }`}
                        />
                        {usuario.ativo ? 'Sim' : 'Não'}
                      </span>
                    </td>
                    <td className='py-3.5 px-6 text-left'>
                      <span
                        className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ring-1 ring-inset ${
                          statusStyle[usuario.statusFinanceiro ?? ''] ??
                          'bg-gray-100 text-gray-600 ring-gray-500/20'
                        }`}
                      >
                        {usuario.statusFinanceiro}
                      </span>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};

export default Home;
