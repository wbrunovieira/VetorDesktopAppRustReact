import React, { useState } from 'react';

import { Link, useNavigate } from 'react-router-dom';

interface User {
  nome: string;
  cpf: string;
  grupo: string;
  ativo: boolean;
  statusFinanceiro: string;
}

const dadosUsuarios: User[] = [
  {
    nome: 'Ana Silva',
    cpf: '123.456.789-00',
    grupo: 'Admin',
    ativo: true,
    statusFinanceiro: 'Em dia',
  },
  {
    nome: 'Bruno Martins',
    cpf: '987.654.321-11',
    grupo: 'Usuário',
    ativo: true,
    statusFinanceiro: 'Em atraso',
  },
  {
    nome: 'Carlos Eduardo',
    cpf: '111.222.333-44',
    grupo: 'Moderador',
    ativo: false,
    statusFinanceiro: 'Em dia',
  },
  {
    nome: 'Diana Rocha',
    cpf: '555.666.777-88',
    grupo: 'Admin',
    ativo: true,
    statusFinanceiro: 'Em atraso',
  },
  {
    nome: 'Eduardo Lima',
    cpf: '999.888.777-66',
    grupo: 'Usuário',
    ativo: false,
    statusFinanceiro: 'Em dia',
  },
  {
    nome: 'Fernanda Gomes',
    cpf: '444.555.666-77',
    grupo: 'Moderador',
    ativo: true,
    statusFinanceiro: 'Em dia',
  },
  {
    nome: 'Gustavo Pereira',
    cpf: '222.333.444-55',
    grupo: 'Admin',
    ativo: true,
    statusFinanceiro: 'Em atraso',
  },
  {
    nome: 'Helena Souza',
    cpf: '666.777.888-99',
    grupo: 'Usuário',
    ativo: false,
    statusFinanceiro: 'Em dia',
  },
  {
    nome: 'Igor Andrade',
    cpf: '000.111.222-33',
    grupo: 'Moderador',
    ativo: true,
    statusFinanceiro: 'Em atraso',
  },
  {
    nome: 'Juliana Vieira',
    cpf: '888.999.000-11',
    grupo: 'Admin',
    ativo: true,
    statusFinanceiro: 'Em dia',
  },
  // Adicione os outros usuários aqui...
];

const Home: React.FC = () => {
  const [ano, setAno] = useState('');
  const [nome, setNome] = useState('');
  const [cpf, setCpf] = useState('');
  const [usuarios, setUsuarios] = useState<User[]>(dadosUsuarios);

  const navigate = useNavigate();

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
  };

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
              {usuarios.map((usuario, index) => (
                <tr key={index} className='bg-primary-light border-b'>
                  <th
                    scope='row'
                    className='py-4 px-6 font-medium text-gray-900 whitespace-nowrap'
                  >
                    {usuario.nome}
                  </th>
                  <td className='py-4 px-6'>{usuario.cpf}</td>
                  <td className='py-4 px-6'>{usuario.grupo}</td>
                  <td className='py-4 px-6'>{usuario.ativo ? 'Sim' : 'Não'}</td>
                  <td className='py-4 px-6'>{usuario.statusFinanceiro}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};

export default Home;
