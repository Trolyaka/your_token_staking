import {
    PublicKey,
    SystemProgram,
    Transaction,
    TransactionInstruction,
} from '@solana/web3.js';
import { getUserStorageAccountWithNonce } from '../utils';
import { Pubkeys } from '../constants';
import { ConnectionService } from '../config';
import BN from 'bn.js';
export async function createUserTransaction(
    userWallet: PublicKey
): Promise<Transaction> {
    const connection = ConnectionService.getConnection();

    const [userStoragePubkey, nonce] = await getUserStorageAccountWithNonce(
        userWallet
    );
    console.log("userStoragePubkey",userStoragePubkey)
    const createUserIx = new TransactionInstruction({
        programId: Pubkeys.cwarStakingProgramId,
        keys: [
            {
                pubkey: userWallet,
                isSigner: true,
                isWritable: false,
            },

            {
                pubkey: userStoragePubkey,
                isSigner: false,
                isWritable: true,
            },

            {
                pubkey: Pubkeys.cwarPoolStoragePubkey,
                isSigner: false,
                isWritable: true,
            },

            {
                pubkey: SystemProgram.programId,
                isSigner: false,
                isWritable: false,
            },
        ],
        data: Buffer.from([
            1, ...new BN(nonce.valueOf()).toArray('le', 1)
        ]),
    });
    const createUserTx = new Transaction().add(createUserIx);
    createUserTx.recentBlockhash = (
        await connection.getRecentBlockhash()
    ).blockhash;
    createUserTx.feePayer = userWallet;

    return createUserTx;
}
