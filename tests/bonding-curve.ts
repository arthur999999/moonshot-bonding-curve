import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BondingCurve } from "../target/types/bonding_curve";

describe("bonding-curve", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BondingCurve as Program<BondingCurve>;

  it("Test ConfigInit instruction", async () => {

    const provider = anchor.getProvider();
    const user = provider.publicKey;

    const configParams = {
      migrationAuthority: anchor.web3.Keypair.generate().publicKey,                   
      backendAuthority: anchor.web3.Keypair.generate().publicKey, 
      configAuthority: user,                  
      helioFee: anchor.web3.Keypair.generate().publicKey,        
      dexFee: anchor.web3.Keypair.generate().publicKey,                              
      feeBps: 100,                             
      dexFeeShare: 60,                            
      migrationFee: new anchor.BN(4),                      
      marketcapThreshold: new anchor.BN(345000000000),                  
      marketcapCurrency: 0,                      
      minSupportedDecimalPlaces: 6,             
      maxSupportedDecimalPlaces: 9,             
      minSupportedTokenSupply: new anchor.BN(10000000),       
      maxSupportedTokenSupply: new anchor.BN(1000000000),    
      coefB: 25,                                
    };

    const tx = await program.methods.configInit(configParams).accounts({
      configAuthority: user,
  
    }).rpc();
    console.log("Tx signature", tx);

    let account = await program.account.configAccount.all();

    console.log("Config account created", account);
    
  });
});
