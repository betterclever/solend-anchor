import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Example } from '../target/types/example';

describe('example', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Example as Program<Example>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
