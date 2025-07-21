import { useQuery } from 'react-query';
import { api } from '../api/client';
import { Escrow } from '../types';

export function useEscrows() {
  return useQuery<Escrow[]>('escrows', () => api.get('/api/v1/escrows').then((r) => r.data));
}
