fn main() {}

/*
Hi Dylan,

Sorry for the delay, i was having a very bad viral fever.
Here is step by step solution of your question

Destructured Requirements (my understanding):

1. off-chain to on-chain message transfer
2. Describe Tools and methods for implementing above feature using
    a. pallets
    b. contracts
    c. bridges

Prerequisite:

 - Setup Substrate Node template and run it in dev mode
 - Setup an off-chain worker
        - off-chain workers are typically used for tasks that do not need to be executed on the blockchain itself

Solution 1 : Using pallets

a. first of all setup a custom pallet
   - A simple pallet demonstrating concepts, APIs and structures common to most offchain workers.
b. Inside the pallet, we can import, the off-chain supporting modules from the frame_system and sp_runtime crate
      for ex :
      use frame_system::{offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
		SignedPayload, Signer, SigningTypes, SubmitTransaction,
	},

	use sp_runtime::{ offchain::{
		http,
		storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
		Duration,
	},

c. Then setup a #[pallet::hooks] , for ex: fn off-chain_worker(block_number: BlockNumberFor<T>)
    -  By implementing `fn off-chain_worker` you declare a new off-chain worker.
	   This function will be called when the node is fully synced and a new best block is successfully imported.
	   Note that it's not guaranteed for off-chain workers to run on EVERY block, there might be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
	   so the code should be able to handle that. You can use `Local Storage` API to coordinate runs of the worker.

d. Setup the #[pallet::call] functionalities, which will be invoked by the  off-chain-worker hook , through signed or unsigned transaction
e. By using above method, we can transfer message from off-chain worked to on-chain runtime using pallets

Note: Above is a minimalistic explanation, i haven't mentioned the off-chain storage, or off-chain indexing (where runtime can directly interact with offchain storage.
If you want to explore more, you can use this example pallet for off-chain worker link:
https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame/examples/offchain-worker

More solution will be updated soon...

 */