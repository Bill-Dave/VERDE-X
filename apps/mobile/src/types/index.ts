export interface Escrow {
  id: string;
  asset: string;
  amount: number;
  initiator_id: string;
  counterparty_id: string;
  status: 'pending' | 'funded' | 'released' | 'disputed' | 'completed';
  created_at: string;
}

export interface User {
  id: string;
  email: string;
  kyc_status: 'none' | 'pending' | 'verified';
}
