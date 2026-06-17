import React, { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
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
  const navigate = useNavigate();
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

  const brl = (v: string) => {
    const n = Number((v ?? '').trim().replace(/\./g, '').replace(',', '.'));
    return isNaN(n)
      ? '—'
      : n.toLocaleString('pt-BR', { style: 'currency', currency: 'BRL' });
  };

  const initials = user.nome
    .split(' ')
    .filter(Boolean)
    .slice(0, 2)
    .map((w) => w[0])
    .join('')
    .toUpperCase();

  const rendimentos = [
    { label: 'Rendimentos Tributáveis', value: user.rend_tributaveis },
    { label: 'Rendimentos Isentos', value: user.rend_isentos },
    { label: 'Tributação Exclusiva', value: user.rend_exclusivos },
  ];
  const pagamentos = [
    { label: 'Total de Juros (anual)', value: user.juros },
    { label: 'Doações a Partidos Políticos', value: user.doacoes_politicas },
    { label: 'Pagamentos / Doações / Outros', value: user.pagamentos_doacoes_outros },
  ];

  const Tile = ({ label, value }: { label: string; value: string }) => (
    <div className='rounded-xl bg-white border border-primary-ligher p-4 shadow-sm transition-shadow hover:shadow-md'>
      <p className='text-[11px] font-semibold uppercase tracking-wide text-primary'>
        {label}
      </p>
      <p className='mt-1.5 text-xl font-semibold text-primary-almostBlack tabular-nums'>
        {brl(value)}
      </p>
    </div>
  );

  return (
    <div className='flex flex-col items-center justify-center min-h-screen bg-gradient-to-r from-primary-dark via-primary-light to-primary-dark p-6'>
      <div className='max-w-4xl w-full bg-primary-moreLighter shadow-2xl rounded-2xl overflow-hidden'>
        {/* Header */}
        <div className='bg-primary-dark px-8 py-6 flex items-center gap-5'>
          <div className='flex h-16 w-16 shrink-0 items-center justify-center rounded-full bg-white/15 text-2xl font-bold text-white ring-2 ring-white/30'>
            {initials}
          </div>
          <div className='min-w-0 flex-1'>
            <h1 className='truncate text-2xl font-bold text-white'>
              {user.nome}
            </h1>
            <div className='mt-1 flex flex-wrap items-center gap-x-3 gap-y-1 text-sm text-primary-ligher'>
              <span className='tabular-nums'>CPF {user.cpf}</span>
              <span className='inline-flex items-center rounded-full bg-white/15 px-2.5 py-0.5 text-xs font-medium text-white'>
                Exercício {user.exercicio}
              </span>
            </div>
          </div>
          <button
            type='button'
            className='shrink-0 rounded-lg bg-white/15 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-white/25'
            onClick={() => navigate('/home')}
          >
            ← Voltar
          </button>
        </div>

        {/* Body */}
        <div className='px-8 py-7'>
          <h2 className='mb-3 text-sm font-bold uppercase tracking-wider text-primary-darker'>
            Rendimentos
          </h2>
          <div className='grid grid-cols-1 gap-4 sm:grid-cols-3'>
            {rendimentos.map((r) => (
              <Tile key={r.label} label={r.label} value={r.value} />
            ))}
          </div>

          <h2 className='mb-3 mt-7 text-sm font-bold uppercase tracking-wider text-primary-darker'>
            Pagamentos e Deduções
          </h2>
          <div className='grid grid-cols-1 gap-4 sm:grid-cols-3'>
            {pagamentos.map((p) => (
              <Tile key={p.label} label={p.label} value={p.value} />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default UserDetails;
