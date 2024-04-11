import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/tauri';
interface User {
  cpf: string;
  nome: string;
  exercicio: string;
  rend_tributaveis: string;
  rend_isentos: string;
  rend_exclusivos: string;
  juros: string;
  doacoes_politicas: string;
  pagamentos_doacoes_outros: string;
}

const UserDetails: React.FC = () => {
  const { cpf } = useParams<{ cpf: string }>();
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    const fetchUser = async () => {
      try {
        const fetchedUser: User = await invoke('get_user_by_cpf', { cpf });
        console.log('cpf', cpf);
        setUser(fetchedUser);
      } catch (error) {
        console.error('Erro ao buscar detalhes do usuário:', error);
        setError('Falha ao carregar detalhes do usuário.');
      } finally {
        setIsLoading(false);
      }
    };

    fetchUser();
  }, [cpf]);

  if (error) {
    return <div className='error'>{error}</div>;
  }

  if (isLoading) {
    return <div>Carregando...</div>;
  }

  if (!user) {
    return <div>Usuário não encontrado.</div>;
  }

  return (
    <div className='flex flex-col items-center justify-center min-h-screen bg-gradient-to-r from-primary-dark via-primary-light to-primary-dark'>
      <div className='max-w-4xl w-full px-6 py-8 bg-primary-moreLighter shadow-md rounded-md h-[70%]'>
        <h1 className='text-3xl text-center text-primary-almostBlack mb-8'>
          Detalhes do Usuário
        </h1>
        <div className='grid grid-cols-2 gap-4'>
          <div>
            <p>
              <strong>CPF:</strong> {user.cpf}
            </p>
            <p>
              <strong>Nome:</strong> {user.nome}
            </p>
            <p>
              <strong>Exercício:</strong> {user.exercicio}
            </p>
            <p>
              <strong>Rendimentos Tributáveis:</strong> {user.rend_tributaveis}
            </p>
            <p>
              <strong>Rendimentos Isentos:</strong> {user.rend_isentos}
            </p>
          </div>
          <div>
            <p>
              <strong>Rendimentos Sujeitos à Tributação Exclusiva:</strong>{' '}
              {user.rend_exclusivos}
            </p>
            <p>
              <strong>Pagamento Anual Total de Juros:</strong> {user.juros}
            </p>
            <p>
              <strong>Doações a Partidos Políticos:</strong>{' '}
              {user.doacoes_politicas}
            </p>
            <p>
              <strong>Pagamentos/Doações/Outros:</strong>{' '}
              {user.pagamentos_doacoes_outros}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default UserDetails;
