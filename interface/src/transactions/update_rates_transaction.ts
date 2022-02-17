import {
    PublicKey,
    Transaction,
    TransactionInstruction,
} from '@solana/web3.js';
import { Pubkeys } from '../constants';
import { ConnectionService } from '../config';
import { YourStakingInstructions } from '../models';
import BN from 'bn.js';
export async function createUpdateRatesTransaction(
    poolOwnerPubkey: PublicKey,
    rewards_per_slot,
    max_reward_rate,
    min_reward_rate,
): Promise<Transaction> {
    const connection = ConnectionService.getConnection();
    const createUserIx = new TransactionInstruction({
        programId: Pubkeys.yourStakingProgramId,
        keys: [
            {
                pubkey: poolOwnerPubkey,
                isSigner: true,
                isWritable: false,
            },
            {
                pubkey: Pubkeys.yourPoolStoragePubkey,
                isSigner: false,
                isWritable: true,
            },
        ],
        data: Buffer.from([
            YourStakingInstructions.UpdateRates,
            ...new BN(rewards_per_slot).toArray('le', 8),
            ...new BN(max_reward_rate).toArray('le', 8),
            ...new BN(min_reward_rate).toArray('le', 8),
        ]),
    });
    const createUserTx = new Transaction().add(createUserIx);
    createUserTx.recentBlockhash = (
        await connection.getRecentBlockhash()
    ).blockhash;
    createUserTx.feePayer = poolOwnerPubkey;

    return createUserTx;
}
