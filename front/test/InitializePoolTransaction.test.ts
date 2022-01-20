import {getAdminAccount, requestAirdrop, setupTest} from './testHelpers';
import {Keypair, sendAndConfirmTransaction} from '@solana/web3.js';
import {createInitializePoolTransaction} from '../transactions';
import {ConnectionService} from '../config';
import {Constants, Pubkeys} from '../constants';
import {Token, TOKEN_PROGRAM_ID} from '@solana/spl-token';

setupTest();

describe('Initialize Cwar Pool Transaction', () => {
  let adminAccount: Keypair = getAdminAccount();
  let cwarPoolStorageAccount: Keypair;
  let cwarStakingVault: Keypair;
  let cwarRewardsVault: Keypair;
  let rewardDurationInDays: number;
  const cwarDecimals = 9;
  const rewardTokenDecimals = 9;
  beforeEach(async () => {
    Constants.cwarDecimals = cwarDecimals;
    Constants.rewardTokenDecimals = rewardTokenDecimals;
    // pool owner wallet == admin
    const connection = ConnectionService.getConnection();

    // Create Cwar Test Token
    const cwarTokenMint = await Token.createMint(
      connection,
      adminAccount,
      adminAccount.publicKey,
      null,
      cwarDecimals,
      TOKEN_PROGRAM_ID
    );
    Pubkeys.stakingMintPubkey = cwarTokenMint.publicKey;
    Pubkeys.cwarTokenMintPubkey = cwarTokenMint.publicKey;

    // Create Reward Test Token
    const rewardTokenMint = await Token.createMint(
      connection,
      adminAccount,
      adminAccount.publicKey,
      null,
      rewardTokenDecimals,
      TOKEN_PROGRAM_ID
    );
    Pubkeys.rewardsMintPubkey = rewardTokenMint.publicKey;

    cwarPoolStorageAccount = Keypair.generate();
    cwarStakingVault = Keypair.generate();
    cwarRewardsVault = Keypair.generate();
    rewardDurationInDays = 1;
    await requestAirdrop(adminAccount.publicKey);

    Pubkeys.cwarStakingVaultPubkey = cwarStakingVault.publicKey;
    Pubkeys.cwarRewardsVaultPubkey = cwarRewardsVault.publicKey;
  });

  test('Initialize Pool', async () => {
    const connection = ConnectionService.getConnection();
    const initializePoolTx = await createInitializePoolTransaction(
      adminAccount.publicKey,
      cwarPoolStorageAccount,
      cwarStakingVault,
      cwarRewardsVault,
      rewardDurationInDays
    );
    await sendAndConfirmTransaction(connection, initializePoolTx, [
      adminAccount,
      cwarPoolStorageAccount,
      cwarStakingVault,
      cwarRewardsVault,
    ]);
    Pubkeys.cwarPoolStoragePubkey = cwarPoolStorageAccount.publicKey;
  });
});
